
```
ssh alice@prod 'sudo systemctl status deleteec2.service'
ssh alice@prod 'sudo systemctl status deleteec2.timer'

ssh alice@prod 'sudo systemctl stop deleteec2.service'
ssh alice@prod 'sudo systemctl stop deleteec2.timer'
ssh alice@prod 'sudo systemctl start deleteec2.timer'

ssh alice@prod 'sudo journalctl -xeu deleteec2.service -f'
ssh alice@prod 'nix run github:r33drichards/delete_ec2 --refresh'

nix build
nix sign-paths --key-file key `readlink result` 
nix copy --to ssh-ng://alice@bcache `readlink result`
```

➜  delete_ec2 git:(master) ssh alice@prod 'nix run github:r33drichards/delete_ec2 --refresh'

this derivation will be built:
  /nix/store/ykawckxdpsmipch7ynn19vz5q5dnh0fx-app-0.0.1.drv


➜  delete_ec2 git:(master) ✗ readlink result                                                  
/nix/store/s387j4zc2l96py7mqy5c7w2bj0spxfjy-app-0.0.1