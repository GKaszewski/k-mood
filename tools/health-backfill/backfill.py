"""Backfills k-mood's daily metrics from an Apple Health export.

Reads `export.xml` straight out of the export zip, a record at a time, so the
size of the export does not matter. Each day is worked out with the same rules
as the daily iOS shortcut (tools/ios-shortcut), so history and daily syncs agree.

    python3 backfill.py export.zip            # summary only, sends nothing
    python3 backfill.py export.zip --send     # sends to k-mood

Sending uses the API token stored in the macOS Keychain under the service
"kmood-shortcut". Re-sending a day overwrites it with the same values, and a
value entered by hand in k-mood always wins over an imported one.
"""

import argparse
import json
import subprocess
import sys
import urllib.error
import urllib.request
import xml.etree.ElementTree as ElementTree
import zipfile
from collections import defaultdict
from datetime import date, datetime, timedelta

SERVER = "https://mood.gabrielkaszewski.dev"
IMPORT_PATH = "/api/v1/metrics/import"
KEYCHAIN_SERVICE = "kmood-shortcut"
EXPORT_MEMBER = "apple_health_export/export.xml"
DAYS_PER_REQUEST = 90
# Cloudflare in front of the server refuses Python's default User-Agent
# (error 1010), so the script names itself.
USER_AGENT = "k-mood-health-backfill/1.0"

HEALTH_DATE = "%Y-%m-%d %H:%M:%S %z"
WATCH = "Watch"
OTHER_DEVICES = "other"

STEPS = "HKQuantityTypeIdentifierStepCount"
EXERCISE = "HKQuantityTypeIdentifierAppleExerciseTime"
RESTING_HEART_RATE = "HKQuantityTypeIdentifierRestingHeartRate"
HRV = "HKQuantityTypeIdentifierHeartRateVariabilitySDNN"
SLEEP = "HKCategoryTypeIdentifierSleepAnalysis"

ASLEEP_PREFIX = "HKCategoryValueSleepAnalysisAsleep"
AWAKE = "HKCategoryValueSleepAnalysisAwake"
# A night counts for the day its sleep ends on: shifting the end by this much
# keeps everything that ends before 18:00 on that day.
NIGHT_SHIFT = timedelta(hours=6)
SECONDS_PER_MINUTE = 60


def device_of(source_name):
    """iPhone and Watch both count the same walk, so their totals are kept apart."""
    return WATCH if WATCH in source_name else OTHER_DEVICES


class DailyMetrics:
    def __init__(self):
        self.steps = defaultdict(lambda: defaultdict(float))
        self.exercise = defaultdict(float)
        self.resting_heart_rate = defaultdict(list)
        self.hrv = defaultdict(list)
        self.asleep = defaultdict(lambda: defaultdict(float))
        self.awake = defaultdict(lambda: defaultdict(float))

    def add(self, record):
        kind = record.get("type")
        day = record.get("startDate")[:10]

        if kind == STEPS:
            self.steps[day][device_of(record.get("sourceName", ""))] += float(record.get("value"))
        elif kind == EXERCISE:
            self.exercise[day] += float(record.get("value"))
        elif kind == RESTING_HEART_RATE:
            self.resting_heart_rate[day].append(float(record.get("value")))
        elif kind == HRV:
            self.hrv[day].append(float(record.get("value")))
        elif kind == SLEEP:
            self.add_sleep(record)

    def add_sleep(self, record):
        start = datetime.strptime(record.get("startDate"), HEALTH_DATE)
        end = datetime.strptime(record.get("endDate"), HEALTH_DATE)
        night = (end + NIGHT_SHIFT).date().isoformat()
        source = record.get("sourceName", "")
        minutes = (end - start).total_seconds() / SECONDS_PER_MINUTE
        stage = record.get("value", "")

        if stage.startswith(ASLEEP_PREFIX):
            self.asleep[night][source] += minutes
        elif stage == AWAKE:
            self.awake[night][source] += minutes

    def days(self):
        """`{date: [{"kind", "value"}]}`, leaving out whatever a day has no data for."""
        by_day = defaultdict(list)

        for day, devices in self.steps.items():
            by_day[day].append(metric("steps", max(devices.values())))
        for day, minutes in self.exercise.items():
            by_day[day].append(metric("exerciseMinutes", minutes))
        for day, readings in self.resting_heart_rate.items():
            by_day[day].append(metric("restingHeartRate", average(readings)))
        for day, readings in self.hrv.items():
            by_day[day].append(metric("hrv", average(readings)))
        for night, sources in self.asleep.items():
            # The source that saw the most sleep that night tells the whole night.
            source = max(sources, key=sources.get)
            by_day[night].append(metric("sleepMinutes", sources[source]))
            by_day[night].append(metric("awakeMinutes", self.awake[night].get(source, 0)))

        return dict(sorted(by_day.items()))


def metric(kind, value):
    return {"kind": kind, "value": int(value + 0.5)}


def average(readings):
    return sum(readings) / len(readings)


def read_export(path, since, before):
    """Days in [since, before). Today is left out by default: it is still running,
    and the next daily sync sends it whole."""
    metrics = DailyMetrics()
    with zipfile.ZipFile(path) as archive, archive.open(EXPORT_MEMBER) as export:
        for _, element in ElementTree.iterparse(export, events=("end",)):
            if element.tag == "Record" and since <= element.get("startDate", "")[:10] < before:
                metrics.add(element)
            element.clear()
    return {day: metrics for day, metrics in metrics.days().items() if since <= day < before}


def summarise(days):
    by_year = defaultdict(lambda: defaultdict(int))
    for day, metrics in days.items():
        for entry in metrics:
            by_year[day[:4]][entry["kind"]] += 1

    print(f"{len(days)} days, {next(iter(days), '-')} → {next(reversed(days), '-')}")
    for year, kinds in sorted(by_year.items()):
        print(f"  {year}: " + ", ".join(f"{kind} {count}" for kind, count in sorted(kinds.items())))


def keychain_token():
    found = subprocess.run(
        ["security", "find-generic-password", "-s", KEYCHAIN_SERVICE, "-w"],
        capture_output=True,
        text=True,
    )
    if found.returncode != 0:
        sys.exit(f'no token in the Keychain under "{KEYCHAIN_SERVICE}"; see tools/ios-shortcut/README.md')
    return found.stdout.strip()


def send(days, token):
    entries = [{"date": day, "metrics": metrics} for day, metrics in days.items()]
    accepted = superseded = 0
    rejected = []

    for start in range(0, len(entries), DAYS_PER_REQUEST):
        batch = entries[start : start + DAYS_PER_REQUEST]
        request = urllib.request.Request(
            SERVER + IMPORT_PATH,
            data=json.dumps({"days": batch}).encode(),
            headers={
                "Content-Type": "application/json",
                "Authorization": f"Bearer {token}",
                "User-Agent": USER_AGENT,
            },
            method="POST",
        )
        try:
            with urllib.request.urlopen(request) as response:
                outcome = json.load(response)
        except urllib.error.HTTPError as refusal:
            sys.exit(f"{batch[0]['date']} → {batch[-1]['date']} refused: {refusal.code} {refusal.read().decode()}")

        accepted += outcome["accepted"]
        superseded += outcome["superseded"]
        rejected.extend(outcome["rejected"])
        print(f"  {batch[0]['date']} → {batch[-1]['date']}: accepted {outcome['accepted']}")

    print(f"accepted {accepted}, superseded by hand-entered values {superseded}, rejected {len(rejected)}")
    for rejection in rejected[:20]:
        print("  rejected:", json.dumps(rejection))


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("export", help="the export.zip from Health → profile → Export All Health Data")
    parser.add_argument("--since", default="0000-00-00", help="only days on or after this date, yyyy-MM-dd")
    parser.add_argument(
        "--before", default=date.today().isoformat(), help="only days before this date, yyyy-MM-dd; default today"
    )
    parser.add_argument("--show", metavar="DAY", help="print what would be sent for one day")
    parser.add_argument("--send", action="store_true", help="send to k-mood; without it nothing leaves the machine")
    arguments = parser.parse_args()

    days = read_export(arguments.export, arguments.since, arguments.before)
    summarise(days)
    if arguments.show:
        print(arguments.show, json.dumps(days.get(arguments.show, [])))
    if arguments.send:
        send(days, keychain_token())


if __name__ == "__main__":
    main()
