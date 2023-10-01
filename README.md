# Hash Check

Check a password hash (SHA-1) against a list of hashes of known breached passwords downloaded from the [haveibeenpwned database](https://haveibeenpwned.com/) using the [easypwned](https://github.com/easybill/easypwned) downloader.

## API

Service meta data is available at `GET /`.

### Check

The service provides a single GET endpoint:

```
/:hash
```

Where `:hash` is a SHA-1 hash of the password to check; the reply is a JSON number `1` if the password hash was found in the database otherwise `0`.

## Scripts

### Download

Run the `scripts/download.sh` script to install the downloader, download the database and create the bloom filter.

### Upload

Run the `scripts/upload.sh` script to copy the bloom filter and meta data files to S3.

## Docker

### Build

If you have already downloaded the database then using `-f Dockerfile-local` will be faster as it won't download the bloom filter data from S3.

```
docker build . --tag hashcheck
```

### Run

```
docker run -it --rm --name hashcheck -p 3342:3342 hashcheck
```

### Deployment

Tag and push to run the workflow that deploys the container to ECR:

```
git tag v1.0.0
git push origin v1.0.0
```

### Manual Deployment Notes

Deploy to the container registry.

Create a login password for the container registry:

```
AWS_PROFILE=sos-containers aws ecr-public get-login-password \
  --region us-east-1 | docker login \
  --username AWS --password-stdin public.ecr.aws/s2y4a9f8
```

## MacOS ARM

For MacOS with ARM chips build for the correct deployment architecture and push:

```
docker buildx build --platform=linux/amd64 --push -t public.ecr.aws/s2y4a9f8/hashcheck:latest .
```

## Linux

Build the image:

```
docker build . --tag hashcheck
```

Tag the image:

```
docker tag hashcheck:latest public.ecr.aws/s2y4a9f8/hashcheck:latest
```

Push the image to the registry:

```
docker push public.ecr.aws/s2y4a9f8/hashcheck:latest
```

© Copyright Save Our Secrets Pte Ltd 2022; all rights reserved.
