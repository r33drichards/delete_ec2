
```
ssh alice@prod 'sudo systemctl status deleteec2.service'
ssh alice@prod 'sudo journalctl -xeu deleteec2.service -f'
ssh alice@prod 'nix run github:r33drichards/delete_ec2 --refresh'
```