# Power Monitor

Simply reads the current state of the device's power consumption from the sysfs file at `/sys/class/power_supply/<BAT0|BAT1>/power_now` and prints the result in watts to 2 decimal places. When the device is plugged in and the battery is charged, it will simply display print `⚡️ AC`.

I use this in conjuction with the Gnome Extension [Executor](https://extensions.gnome.org/extension/2932/executor/) to show the current power consumption in the Gnome top bar.

![image](https://github.com/vincentbavitz/power-monitor/assets/58160433/419acc59-e221-4ffe-b273-a48c144d9150)
![image](https://github.com/vincentbavitz/power-monitor/assets/58160433/13422e70-575c-4cad-a718-7bd244584c2e)
