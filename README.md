Move Script Runner Cli

Since there's no feature to sign and submit move script on wallet extension, this is a cli program that can execute sign and execute script with given private key adn address

Private key will not be saved to anywhere!

How to run
```shell
run -p {private_key} -a {account address} -b {bytecode}
```