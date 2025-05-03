* [x] `wipd --systemd`: run as a systemd instanced service in ~/.config/systemd
    * [ ] have some way to stop the systemd service of current dir (note: that
          is kinda hard if you have wipd running in direnv)
    * [ ] have a way to stop all wipd instances (if they are running on systemd,
          run systemd stop, if not, just kill). Right now what can be done is
          `killall -9 wip-fd41eca378` (note, `kilalll -9 wipd` doesn't work)
