.PHONY: all clean kotlin

all: kotlin swift docs

kotlin:
	cargo build
	cd kotlin && ./gradlew build

swift:
	cargo build
#	cargo swift package -p ios -n WalletSdkRs

docs: kotlin swift
	mkdir -p docs/rust/api
	mono natural-docs/NaturalDocs.exe \
		-i src \
		-p ./natural-docs-config/ \
		-o html docs/rust/api \

	mkdir -p docs/kotlin/api
	mono natural-docs/NaturalDocs.exe \
		-i kotlin/walletsdkrs/src/main/java/com/spruceid/wallet/sdk/rs/ \
		-p ./natural-docs-config/ \
		-o html docs/kotlin/api

	# mkdir -p docs/swift/api
	# mono natural-docs/NaturalDocs.exe ./natural-docs-config/
