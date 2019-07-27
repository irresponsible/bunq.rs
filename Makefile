SANDBOX_USER_URL := "https://public-api.sandbox.bunq.com/v1/sandbox-user"
JSON := "Content-Type: application/json"
NOCACHE := "Cache-Control: none"
USERAGENT := "User-Agent: curl-request"
LANGUAGE := "X-Bunq-Language: nl_NL"
REGION := "X-Bunq-Region: nl_NL"
GEO := "X-Bunq-Geolocation: 0 0 0 0 000"
REQUESTID := "X-Bunq-Client-Request-Id: $(shell date)randomId"

SANDBOX_TOKEN_FILE := .sandbox-token
SIGNING_KEY_FILE := .signing-key

.PHONY: refresh-sandbox-token

sandbox-token:
	curl $(SANDBOX_USER_URL) -X POST --header $(JSON) --header $(NOCACHE) --header $(USERAGENT) --header $(REQUESTID) --header $(LANGUAGE) --header $(REGION) --header $(GEO) | perl -pe 's/.+?"api_key":"([^"].+)".+/$$1/' > $(SANDBOX_TOKEN_FILE)

signing-key:
	openssl genpkey -algorithm RSA -pkeyopt rsa_keygen_bits:2048 -pkeyopt rsa_keygen_pubexp:65537 | openssl pkcs8 -topk8 -nocrypt -outform der > $(SIGNING_KEY_FILE)
