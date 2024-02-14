
```
ssh alice@prod 'sudo systemctl status deleteec2.service'
ssh alice@prod 'sudo systemctl status deleteec2.timer'

ssh alice@prod 'sudo systemctl stop deleteec2.service'
ssh alice@prod 'sudo systemctl stop deleteec2.timer'
ssh alice@prod 'sudo systemctl start deleteec2.timer'

ssh alice@prod 'sudo journalctl -xeu deleteec2.service -f'
ssh alice@prod 'nix run github:r33drichards/delete_ec2 --refresh'
nix-copy-closure --to alice@bcache  /nix/store/y5lhkvma9vq7ixxjfal9cwv1pvk8hjry-app-0.0.1.drv
nix sign-paths --key-file key `readlink result` 
nix copy --to ssh-ng://alice@bcache `readlink result`
```