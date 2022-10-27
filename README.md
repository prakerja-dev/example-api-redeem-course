# Contoh Aplikasi Integrasi API Redeem Course Untuk Mitra Prakerja (Lembaga Pendidikan)

Source code pada repo ini dijalankan menggunakan CLI, hanya sebatas memberikan contoh bahwa API nya bekerja. Sebelum mengimplementasikan API pada sistem masing-masing, sebaiknya mencoba dan menjalankan contoh pada repo ini. Setelah berhasil, baru-lah dicoba implementasi nya ke dalam sistem, untuk meminimalisir troubleshoot.

## Refference

- https://prakerja.atlassian.net/l/cp/CizN1hwu

## Prerequisite

Pilih bahasa yang Anda gunakan pada folder yang telah disediakan. Sebelum memulai harap mempersiapkan compiler atau runtime bahasa pemograman yang digunakan pada sistem masing-masing.

- Command Line Interface (Terminal, Power Shell, CMD, etc)
- NodeJS (untuk Lembaga Pendidikan yang menggunakan bahasa Javascript maupun Typescript di sisi server)
- PHP minimal 7.4 (untuk Lembaga Pendidikan yang menggunakan bahasa PHP)
- Python3 (untuk Lembaga Pendidikan yang menggunakan bahasa Python)
- Ruby (untuk Lembaga Pendidikan yang menggunakan bahasa Ruby)
- Java (untuk Lembaga Pendidikan yang menggunakan bahasa Java)
- Go (untuk Lembaga Pendidikan yang menggunakan bahasa Go)
- DotNet Core (untuk Lembaga Pendidikan yang menggunakan bahasa C#)

## Step

Masing-masing folder contoh bahasa pemrograman, telah disediakan dua file, yaitu example-redeem-code-status.EXT untuk pengecekan Redeem Code apakah sudah pernah digunakan atau belum, dan example-redeem-code-commit.EXT untuk mendeklarasi Redeem Code untuk digunakan. Pada dasarnya kedua nya sama, hanya saja berbeda pada URL endpoint nya, yang sesuai dengan penggunaan yang dibutuhkan.

Yang perlu dilakukan sebelum menjalankan code examples ini adalah, merubah nilai variable berikut ini pada dua file yang disebutkan sebelumnya:

- client_code (nilai nya diberikan oleh Tim Prakerja ke email masing-masing Lembaga Pendidikan)
- redeem_code (nilai nya di input oleh user, yang didapat ketika melakukan pembelian di Digital Platform, namun untuk keperluan testing Tim Prakerja akan memberikan Redeem Code dummy ke email masing-masing Lembaga Pendidikan)
- key (nilai nya diberikan oleh Tim Prakerja ke email masing-masing Lembaga Pendidikan)
