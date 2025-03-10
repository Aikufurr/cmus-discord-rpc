# cmus-discord-rpc

[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com) 
[![forthebadge](https://forthebadge.com/images/badges/works-on-my-machine.svg)](https://forthebadge.com)

Discord Rich Presence integration for the C* Music Player (`cmus`)


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
      RestartSec=10

      [Install]
      WantedBy=multi-user.target
      
  `CTRL+O [enter] CTRL+X` to save and close
  
  Then you can enable the service to run on startup and run straight away by doing
  
      systemctl --user enable --now cmus-discord-rpc


Discord Rich Presence integriĝo por la C* Muzika Ludilo (`cmus`)


## Konstruaĵo

- Akiru la fontojn. Vi povas aŭ fari tion per klonado de la deponejo aŭ elŝutante arkivon de la deponejo.

- Ŝanĝu vian dosierujon al kie la fontoj estis klonitaj/ekstraktitaj.

- Fine por konstrui, uzu la jenajn komandojn:

  Por senararigado:

      cargo build

  Por produktada uzo:

      cargo build --release

- Vi devus vidi novan dosierujon nomitan `target`. Tie vi povas trovi subdosierujojn por ĉiu el viaj konstruceloj.

- Por uzi tutmonde, konstruu por produktada uzo kaj poste kopiu al via 'local bin'

      sudo cp ./target/release/cmus-discord-rpc /usr/local/bin
      
- Por funkcii ĉe ekfunkciigo, vi povas uzi systemctl

      nano ~/.config/systemd/user/cmus-discord-rpc.service
      
  Algluu la jenon
  
      [Unit]
      Description="Service for cmus-discord-rpc

      [Service]
      ExecStart=/usr/local/bin/cmus-discord-rpc
      Restart=always
      RestartSec=10

      [Install]
      WantedBy=multi-user.target
      
  `CTRL+O [enter] CTRL+X` al savi kaj fermi
  
  Tiam vi povas ebligi la servon funkcii ĉe ekfunkciigo kaj kuri tuj per faro
  
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
