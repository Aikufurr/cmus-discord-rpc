# cmus-discord-rpc

[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com) 
[![forthebadge](https://forthebadge.com/images/badges/works-on-my-machine.svg)](https://forthebadge.com)

Discord Rich Presence integration for the C* Music Player (`cmus`) with album cover support and links to the album for [Lapfox](https://lapfoxtrax.fandom.com) songs.


## Building

- Obtain the sources. You can either do this by cloning the repository or downloading an archive of the repository.

- Change your directory into where the sources were cloned/extracted to.

- Finally to build, use the following commands:

  For debugging:

      cargo build

  For production use:

      cargo build --release

- You should see a new directory called `target`. There you can find subfolders for each of your build targets.

- To use globally, build for production use and then copy to your local bin

      sudo cp ./target/release/cmus-discord-rpc /usr/local/bin
      
- To run on startup, you can use systemctl

      nano ~/.config/systemd/user/cmus-discord-rpc.service
      
  Paste in the following
  
      [Unit]
      Description="Service for cmus-discord-rpc

      [Service]
      ExecStart=/usr/local/bin/cmus-discord-rpc
      Restart=always

      [Install]
      WantedBy=multi-user.target
      
  `CTRL+O [enter] CTRL+X` to save and close
  
  Then you can enable the service to run on startup and run straight away by doing
  
      systemctl --user enable --now cmus-discord-rpc

## License

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see https://www.gnu.org/licenses/.
