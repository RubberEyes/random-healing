# random-healing
Run this program on your prefered terminal emulator or PowerShell if you are on windows
It will prompt you for your Medicine Bonus and after receiving it, it will simulate a lot of dice rolls for all treat wounds and risky surgeries DCs as well as the amount they heal based on the checks
This program will create a folder and files inside that folder named "risky_healing_$SKILLNUMBER"
Cram those csv files into Excel or Libre Calc or whatever, and draw your own conclusions at how effective you are at healing

This is 100% rust, so you can compile it in any system simply by getting the source and doing the good old "cargo build --release"
I strongly recommend building as --release as it will run much faster than the default building option
This is a very CPU intensive program and uses multi-threading, so it will easily put you CPU to 100% while it runs.
If you want it run even faster at the cost ofthe accuracy of the data set, then edit the "cont HUGE" to a smaller amount. deleting a zero should be enough.
