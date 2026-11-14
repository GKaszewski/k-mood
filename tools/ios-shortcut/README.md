# k-mood Health sync (iOS Shortcut)

Generates a signed Apple Shortcut that sends the last three days of Health data
to `POST /api/v1/metrics/import`, so it can run unattended every morning.

Sent per day, when Health has it:

| k-mood metric      | From Health                                                         |
| ------------------ | ------------------------------------------------------------------- |
| `steps`            | the larger of the iPhone's and the Watch's totals — summing them double-counts |
| `exerciseMinutes`  | Exercise Minutes, summed                                            |
| `restingHeartRate` | Resting Heart Rate, averaged                                        |
| `hrv`              | Heart Rate Variability, averaged, in ms                             |
| `sleepMinutes`     | Asleep (Core, Deep, REM) in the night that ends on that day         |
| `awakeMinutes`     | Awake within that night                                             |

Sleep is off by default (`SYNC_SLEEP` in `kmood_health.py`): it only exists on
nights the Watch is worn, and a search that finds nothing stops the whole run.
When on, a night belongs to the day it ends on, counting sleep that ends before
18:00.
A day without data for a metric leaves that metric out rather than inventing a
value.
Screen time is not readable from Shortcuts and drinks are not logged in Health, so
both stay manual entries in k-mood.

## Build

1. In k-mood, create an API token (Settings → API tokens).
2. Store it in the macOS Keychain once — it prompts, so the token never lands in
   shell history:

   ```sh
   security add-generic-password -s kmood-shortcut -a kmood -w
   ```

3. Build and sign:

   ```sh
   tools/ios-shortcut/build.sh
   ```

   The signed file lands on the Desktop. Without a Keychain entry, the
   shortcut asks for the token when it is added instead.

The server address is `SERVER` in `kmood_health.py`.

## Install on the iPhone

1. AirDrop the file over and tap **Add Shortcut**.
2. Health → profile → Apps → Shortcuts: allow **reading** Steps, Sleep, Exercise
   Minutes, Resting Heart Rate and Heart Rate Variability.
   A type without read access silently comes back empty.
3. Run it once by hand and allow the connection to the server.
4. Shortcuts → Automation → **Time of Day**, 09:00, Daily, **Run Immediately**,
   and pick the shortcut.

The notification lists how many samples each Health type returned and the
server's reply, which counts accepted and rejected values.

## Changing it

`shortcut_kit.py` holds the plist shapes, each checked against shortcuts
exported from iOS; `kmood_health.py` is the shortcut itself, one function per
metric. Health type names (`STEPS`, `HRV`, …) are the values Find Health Samples
stores, which are not always the label its picker shows ("Exercise Minutes" is
stored as `Exercise Time`). If a type reports `0` found despite read access and
its Unit menu is empty on the phone, the stored name is wrong: build a one-action
shortcut on the phone, share it as an iCloud link, and read the exact value from
`https://www.icloud.com/shortcuts/api/records/<id>`.
