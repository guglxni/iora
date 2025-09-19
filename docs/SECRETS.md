# Secrets with SOPS (age)
1) Generate an age key: `age-keygen -o .agekey`
2) Put your public key in `.sops.yaml` (replace the placeholder).
3) Edit `helm/fedzk/secrets.enc.yaml` with real values, then encrypt:
   `sops -e -i helm/fedzk/secrets.enc.yaml`
4) Decrypt for local dev: `sops -d helm/fedzk/secrets.enc.yaml > /tmp/secrets.yaml`
5) Never commit decrypted secrets.
