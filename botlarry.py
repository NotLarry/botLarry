#!/home/botlarry/phonehome/bin/python3
"""

  botlarry.py initial entry point for botlarry.  At this time it is assumed this will be the code started by systemctl at startup and will continue to run.

  ### build out a unit file so that this always runs

"""

from datetime import datetime

# datetime.today().strftime('%Y%m%d%H%M%s')
filename = datetime.today().strftime('%Y%m%d%H%M%S')+'.mp3'
print(filename)

