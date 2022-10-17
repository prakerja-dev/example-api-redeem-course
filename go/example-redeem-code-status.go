package main

import (
	"bytes"
	"context"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"strconv"
	"time"

	signature_go "example-hmac-sha1-signkey/signature_go"
)

func main() {
	client_code := ""
	key := ""
	redeem_code := ""
	timestamp := time.Now().Unix()

	sign_repo := signature_go.NewPlatformSignature(client_code, key)
	sign_timestamp := time.Unix(timestamp, 0)
	signature, err := sign_repo.GenerateForRedeemStatus(context.TODO(), sign_timestamp, redeem_code)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Printf("timestamp: %s\n", strconv.Itoa(int(timestamp)))
	fmt.Println("signature: " + signature)

	url := "https://api-ssn.prakerja.go.id/api/v1/integration/payment/redeem-code/status"

	var data = []byte(fmt.Sprintf(`{"redeem_code":"%s"}`, redeem_code))
	req, err := http.NewRequest("POST", url, bytes.NewBuffer(data))

	header_timestamp := strconv.Itoa(int(timestamp))
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Client_code", client_code)
	req.Header.Set("Signature", signature)
	req.Header.Set("Timestamp", header_timestamp)

	client := &http.Client{}
	result, err := client.Do(req)
	if err != nil {
		panic(err)
	}
	defer result.Body.Close()

	body, _ := ioutil.ReadAll(result.Body)
	fmt.Println("response:", string(body))
}
