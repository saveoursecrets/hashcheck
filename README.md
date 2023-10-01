# Hash Check

Check a password hash (SHA-1) against a list of hashes of known breached password downloaded from the [haveibeenpwned database](https://haveibeenpwned.com/) using the [easypwned](https://github.com/easybill/easypwned) downloader.

## Scripts

### Download

Run the `scripts/download.sh` script to install the downloader and download the database and create the bloom filter.

### Upload

Run the `scripts/upload.sh` script to copy the downloaded database and bloom filter to S3.

## Docker

### Build

```
docker build . --tag hashcheck
```

### Run

```
docker run -it --rm --name hashcheck -p 3342:3342 hashcheck
```

© Copyright Save Our Secrets Pte Ltd 2022; all rights reserved.
