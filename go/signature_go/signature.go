package signature_go

import (
	"context"
	"crypto/hmac"
	"crypto/sha1"
	"encoding/hex"
	"fmt"
	"time"
)

type SignatureCheck struct {
	Method        string
	Timestamp     int64
	Authorization string
	Body          []byte
	Endpoint      string
}

type PlatformSignature interface {
	GenerateForRedeemCommit(ctx context.Context, t time.Time, redeemCode string) (string, error)
	GenerateForRedeemStatus(ctx context.Context, t time.Time, redeemCode string) (string, error)
}

type platformSignature struct {
	clientCode string
	credential string
}

func NewPlatformSignature(clientCode string, credential string) PlatformSignature {
	return &platformSignature{clientCode: clientCode, credential: credential}
}

func (p platformSignature) GenerateForRedeemStatus(ctx context.Context, t time.Time, redeemCode string) (string, error) {
	return p.generate(ctx, SignatureCheck{
		Method:        "POST",
		Timestamp:     t.Unix(),
		Authorization: "",
		Body:          []byte(fmt.Sprintf(`{"redeem_code":"%s"}`, redeemCode)),
		Endpoint:      "/api/v1/integration/payment/redeem-code/status",
	})
}

func (p platformSignature) GenerateForRedeemCommit(ctx context.Context, t time.Time, redeemCode string) (string, error) {
	return p.generate(ctx, SignatureCheck{
		Method:        "POST",
		Timestamp:     t.Unix(),
		Authorization: "",
		Body:          []byte(fmt.Sprintf(`{"redeem_code":"%s"}`, redeemCode)),
		Endpoint:      "/api/v1/integration/payment/redeem-code/commit",
	})
}

func (p platformSignature) generate(ctx context.Context, data SignatureCheck) (string, error) {
	return EncodeSHA1HMAC(p.credential, p.clientCode, data.Timestamp, data.Method, data.Endpoint, data.Authorization, string(data.Body))
}

func EncodeSHA1HMAC(key string, data ...interface{}) (string, error) {
	mac := hmac.New(sha1.New, []byte(key))
	str := ""
	for _, v := range data {
		str = fmt.Sprintf("%v%v", str, v)
	}

	_, err := mac.Write([]byte(str))
	if err != nil {
		return "", fmt.Errorf("error platformSignature.encodeSHA1HMAC : %v", err)
	}

	expectedMAC := mac.Sum(nil)
	return hex.EncodeToString(expectedMAC[:]), nil
}
