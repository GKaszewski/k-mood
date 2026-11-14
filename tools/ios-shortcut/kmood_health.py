"""Generates the "k-mood Health sync" shortcut.

Each run sends the last three days of Health data to POST /api/v1/metrics/import.
Re-sending a day overwrites it with the same values, so a missed morning is
filled in by the next one.

    python3 kmood_health.py <output.shortcut>

The k-mood address is built in. The API token is read from the macOS Keychain
(service "kmood-shortcut") so it goes from the Keychain straight into the file;
without it, the shortcut asks for the token when it is added.
"""

import plistlib
import subprocess
import sys

from shortcut_kit import CONTAINS, IS, Shortcut, attachment, headers, output, text, variable

SERVER = "https://mood.gabrielkaszewski.dev"
KEYCHAIN_SERVICE = "kmood-shortcut"
NAME = "k-mood Health sync"

DAYS = 3
SEARCH_DAYS = DAYS + 1
# Sleep only exists on nights the Watch is worn. With it on and no sleep in the
# searched days, the search comes back empty and Shortcuts stops the whole run,
# so it stays off until the Watch is worn to bed most nights.
SYNC_SLEEP = False
WATCH = "Watch"

# Find Health Samples type values, each confirmed on a device. The stored value
# is not always the label the picker shows: the picker says "Exercise Minutes"
# but stores "Exercise Time", and that type also needs its unit spelled out.
# Drinks are not synced: a type never logged in Health makes its search come
# back empty, which Shortcuts treats as an error that stops the run.
STEPS = "Steps"
EXERCISE = "Exercise Time"
EXERCISE_UNIT = "min"
RESTING_HEART_RATE = "Resting Heart Rate"
HRV = "Heart Rate Variability"
SLEEP = "Sleep"

ASLEEP = "Asleep"
AWAKE = "Awake"
# A night counts for the day its sleep ends on; shifting the end by this much
# keeps everything that ends before 18:00 on that day.
NIGHT_SHIFT_HOURS = 6
SECONDS_PER_MINUTE = 60


def day_variable(day):
    return f"Day{day}"


def list_name(metric, day):
    return f"{metric}{day}"


def define_days(shortcut):
    """Day1 is yesterday, Day2 the day before, and so on, as yyyy-MM-dd."""
    dates = [
        output(shortcut.add("date", WFDateActionMode="Specified Date", WFDateActionDate=text("yesterday")), "Date")
    ]
    for _ in range(DAYS - 1):
        earlier = shortcut.adjust_date(dates[-1], "Subtract", 1, "Day", "days")
        dates.append(output(earlier, "Adjusted Date"))
    for day, date in enumerate(dates, start=1):
        formatted = shortcut.format_day(date)
        shortcut.set_variable(day_variable(day), output(formatted, "Formatted Date"))


def for_each_matching_day(shortcut, day_key, body):
    """Runs `body(day)` for whichever of the sent days `day_key` falls on."""
    for day in range(1, DAYS + 1):
        same_day = shortcut.if_text(IS, output(day_key, "Formatted Date"), text(variable(day_variable(day))))
        body(day)
        shortcut.end_if(same_day)


def collect_by_start_day(shortcut, sample_type, metric, unit=None):
    """Appends each sample's value to the list for the day it started on."""
    samples = shortcut.find_health_samples(sample_type, SEARCH_DAYS, unit)
    loop = shortcut.for_each(output(samples, "Health Samples"))
    start = shortcut.sample_detail("Start Date")
    day_key = shortcut.format_day(output(start, "Start Date"))
    value = shortcut.sample_detail("Value")
    for_each_matching_day(
        shortcut, day_key, lambda day: shortcut.append_to(list_name(metric, day), output(value, "Value"))
    )
    shortcut.end_for_each(loop)
    return samples


def collect_steps(shortcut):
    """Steps per day kept apart by device, since iPhone and Watch both count the same walk."""
    samples = shortcut.find_health_samples(STEPS, SEARCH_DAYS)
    loop = shortcut.for_each(output(samples, "Health Samples"))
    start = shortcut.sample_detail("Start Date")
    day_key = shortcut.format_day(output(start, "Start Date"))
    value = shortcut.sample_detail("Value")
    source = shortcut.sample_detail("Source")
    source_name = shortcut.add("gettext", WFTextActionText=text(output(source, "Source")))

    def by_device(day):
        from_watch = shortcut.if_text(CONTAINS, output(source_name, "Text"), text(WATCH))
        shortcut.append_to(list_name("StepsWatch", day), output(value, "Value"))
        shortcut.otherwise(from_watch)
        shortcut.append_to(list_name("StepsPhone", day), output(value, "Value"))
        shortcut.end_if(from_watch)

    for_each_matching_day(shortcut, day_key, by_device)
    shortcut.end_for_each(loop)
    return samples


def collect_sleep(shortcut):
    """Asleep and awake durations per night, a night belonging to the day it ends on."""
    samples = shortcut.find_health_samples(SLEEP, SEARCH_DAYS)
    loop = shortcut.for_each(output(samples, "Health Samples"))
    end = shortcut.sample_detail("End Date")
    shifted = shortcut.adjust_date(output(end, "End Date"), "Add", NIGHT_SHIFT_HOURS, "Hour", "hr")
    day_key = shortcut.format_day(output(shifted, "Adjusted Date"))
    stage = shortcut.sample_detail("Value")
    stage_name = shortcut.add("gettext", WFTextActionText=text(output(stage, "Value")))
    duration = shortcut.sample_detail("Duration")

    def by_stage(day):
        asleep = shortcut.if_text(CONTAINS, output(stage_name, "Text"), text(ASLEEP))
        shortcut.append_to(list_name("Sleep", day), output(duration, "Duration"))
        shortcut.otherwise(asleep)
        awake = shortcut.if_text(IS, output(stage_name, "Text"), text(AWAKE))
        shortcut.append_to(list_name("Awake", day), output(duration, "Duration"))
        shortcut.end_if(awake)
        shortcut.end_if(asleep)

    for_each_matching_day(shortcut, day_key, by_stage)
    shortcut.end_for_each(loop)
    return samples


def add_metric(shortcut, day, kind, values, operation, divide_by=None):
    """Appends `{"kind":…,"value":…}` to the day's metrics when it has any values."""
    count = shortcut.add("count", WFCountType="Items", Input=attachment(variable(values)))
    has_values = shortcut.if_more_than(output(count, "Count"), 0)
    result = shortcut.add("statistics", WFStatisticsOperation=operation, Input=attachment(variable(values)))
    result_name = operation
    if divide_by:
        result = shortcut.add(
            "math",
            WFInput=attachment(output(result, result_name)),
            WFMathOperation="÷",
            WFMathOperand=str(divide_by),
        )
        result_name = "Calculation Result"
    rounded = shortcut.add("round", WFRoundMode="Normal", WFInput=attachment(output(result, result_name)))
    fragment = shortcut.add(
        "gettext",
        WFTextActionText=text('{"kind":"', kind, '","value":', output(rounded, "Rounded Number"), "}"),
    )
    shortcut.append_to(list_name("Metrics", day), output(fragment, "Text"))
    shortcut.end_if(has_values)


def add_steps(shortcut, day):
    """The larger of the two devices' totals."""
    for device in ("StepsWatch", "StepsPhone"):
        count = shortcut.add("count", WFCountType="Items", Input=attachment(variable(list_name(device, day))))
        has_values = shortcut.if_more_than(output(count, "Count"), 0)
        device_total = shortcut.add(
            "statistics", WFStatisticsOperation="Sum", Input=attachment(variable(list_name(device, day)))
        )
        shortcut.append_to(list_name("StepTotals", day), output(device_total, "Sum"))
        shortcut.end_if(has_values)
    add_metric(shortcut, day, "steps", list_name("StepTotals", day), "Maximum")


def add_day(shortcut, day):
    add_steps(shortcut, day)
    add_metric(shortcut, day, "exerciseMinutes", list_name("Exercise", day), "Sum")
    add_metric(shortcut, day, "restingHeartRate", list_name("RestingHeartRate", day), "Average")
    add_metric(shortcut, day, "hrv", list_name("Hrv", day), "Average")
    if SYNC_SLEEP:
        add_metric(shortcut, day, "sleepMinutes", list_name("Sleep", day), "Sum", divide_by=SECONDS_PER_MINUTE)
        add_metric(shortcut, day, "awakeMinutes", list_name("Awake", day), "Sum", divide_by=SECONDS_PER_MINUTE)

    metrics = shortcut.add(
        "text.combine",
        WFTextSeparator="Custom",
        WFTextCustomSeparator=",",
        text=attachment(variable(list_name("Metrics", day))),
    )
    entry = shortcut.add(
        "gettext",
        WFTextActionText=text(
            '{"date":"', variable(day_variable(day)), '","metrics":[', output(metrics, "Combined Text"), "]}"
        ),
    )
    shortcut.append_to("Days", output(entry, "Text"))


def send(shortcut, found):
    days = shortcut.add("text.combine", WFTextSeparator="Custom", WFTextCustomSeparator=",", text=attachment(variable("Days")))
    body = shortcut.add("gettext", WFTextActionText=text('{"days":[', output(days, "Combined Text"), "]}"))
    payload = shortcut.add("detect.dictionary", WFInput=attachment(output(body, "Text")))
    response = shortcut.add(
        "downloadurl",
        WFURL=text(variable("Server"), "/api/v1/metrics/import"),
        WFHTTPMethod="POST",
        WFHTTPBodyType="File",
        Advanced=True,
        ShowHeaders=True,
        WFRequestVariable=attachment(output(payload, "Dictionary")),
        WFHTTPHeaders=headers(
            ("Content-Type", text("application/json")),
            ("Authorization", text("Bearer ", variable("Token"))),
        ),
    )

    counts = []
    for label, samples in found:
        count = shortcut.add("count", WFCountType="Items", Input=attachment(output(samples, "Health Samples")))
        counts.extend([f"{label} ", output(count, "Count"), " · "])
    shortcut.add(
        "notification",
        WFNotificationActionTitle=text(NAME, " ", variable(day_variable(DAYS)), " – ", variable(day_variable(1))),
        WFNotificationActionBody=text(
            output(response, "Contents of URL"), "\nfound: ", *counts
        ),
    )


def build(token):
    shortcut = Shortcut()

    server = shortcut.add("gettext", WFTextActionText=SERVER)
    shortcut.set_variable("Server", output(server, "Text"))
    if token is None:
        shortcut.ask_on_import("An API token from k-mood settings", "kmood_")
    token_text = shortcut.add("gettext", WFTextActionText=token or "kmood_")
    shortcut.set_variable("Token", output(token_text, "Text"))

    define_days(shortcut)

    found = [
        ("steps", collect_steps(shortcut)),
        ("exercise", collect_by_start_day(shortcut, EXERCISE, "Exercise", EXERCISE_UNIT)),
        ("resting HR", collect_by_start_day(shortcut, RESTING_HEART_RATE, "RestingHeartRate")),
        ("HRV", collect_by_start_day(shortcut, HRV, "Hrv")),
    ]
    if SYNC_SLEEP:
        found.append(("sleep", collect_sleep(shortcut)))

    for day in range(1, DAYS + 1):
        add_day(shortcut, day)
    send(shortcut, found)

    return shortcut.as_plist(NAME)


def keychain_token():
    """The token stored with `security add-generic-password -s kmood-shortcut`, if any."""
    found = subprocess.run(
        ["security", "find-generic-password", "-s", KEYCHAIN_SERVICE, "-w"],
        capture_output=True,
        text=True,
    )
    return found.stdout.strip() if found.returncode == 0 else None


if __name__ == "__main__":
    token = keychain_token()
    with open(sys.argv[1], "wb") as target:
        plistlib.dump(build(token), target, fmt=plistlib.FMT_XML)
    print("token: from Keychain" if token else "token: asked for when the shortcut is added")
