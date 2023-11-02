# Power Monitor

Simply reads the current state of the device's power consumption from the sysfs file at `/sys/class/power_supply/<BAT0|BAT1>/power_now` and prints the result in watts to 2 decimal places. When the device is plugged in and the battery is charged, it will simply display print `⚡️ AC`.

I use this in conjuction with the Gnome Extension [Executor](https://extensions.gnome.org/extension/2932/executor/) to show the current power consumption in the Gnome top bar.

![image](https://github.com/vincentbavitz/power-monitor/assets/58160433/97daeea3-2319-4dc7-9c26-153aef1854e8)
