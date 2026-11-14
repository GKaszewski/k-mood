# Apple Health backfill

Sends years of Apple Health history to k-mood in one go — the daily iOS shortcut
(`tools/ios-shortcut`) only covers the last three days.

1. On the iPhone: Health → profile → **Export All Health Data**, and AirDrop the
   `export.zip` to the Mac.
2. Check what would be sent — nothing leaves the machine:

   ```sh
   python3 tools/health-backfill/backfill.py ~/Downloads/export.zip
   python3 tools/health-backfill/backfill.py ~/Downloads/export.zip --show 2026-11-13
   ```

3. Send it. The token comes from the Keychain entry the shortcut build uses
   (`security add-generic-password -s kmood-shortcut -a kmood -w`):

   ```sh
   python3 tools/health-backfill/backfill.py ~/Downloads/export.zip --send
   ```

Days are worked out with the shortcut's rules — the larger of the iPhone's and
the Watch's step totals, summed exercise minutes, averaged resting heart rate
and HRV — so history and daily syncs agree. Unlike the shortcut it also sends
sleep, for the nights the Watch was worn: a night counts for the day it ends on,
taken from the source that recorded the most sleep.

Today is left out (`--before` moves the cut-off); it is still running and the
next daily sync sends it whole. Re-sending is safe: a day is overwritten with
the same values, and anything entered by hand in k-mood wins over an import.
`--since` limits the run to recent days.
