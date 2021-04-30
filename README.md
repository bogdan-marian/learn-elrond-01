# Interaction

## On devnet

Deploy & interact with contract:

```
python3 ./interaction/playground.py --pem=./testnet/wallets/users/alice.pem --proxy=http://localhost:7950
```

Interact with existing contract:

```
python3 ./interaction/playground.py --pem=./testnet/wallets/users/alice.pem --proxy=http://localhost:7950 --contract=erd1...
```

## On testnet

Deploy & interact with contract:

```
python3 ./interaction/playground.py --pem=my.pem --proxy=https://testnet-gateway.elrond.com
```

Interact with existing contract:

```
python3 ./interaction/playground.py --pem=my.pem --proxy=https://testnet-gateway.elrond.com --contract=erd1...
```

## notes from public town hall
```txt
Sever Moldovean
​hey @Bogdan Marian Oloeriu - docs lag behind code, it's quite normal. If you want direct access to devs, go to @ElrondDevelopers on Telegram, tag @SeverMM (me!) and I will expedite your request 4 info
```
[response from public townhall 19 to 25](https://www.youtube.com/watch?v=g6OLh39yOHo)