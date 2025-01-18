# Smart Contract Solana 
Berikut adalah contoh kode smart contract token di Solana dengan fitur yang lebih lengkap. Kode ini mencakup berbagai fungsionalitas seperti pembuatan token, transfer, minting token baru, dan pemusnahan token. Kami akan menggunakan **Anchor** untuk framework dan **Rust** untuk menulis smart contract.

### Fitur yang akan disediakan:
1. **Initialize**: Menyiapkan akun token untuk pengguna.
2. **Transfer**: Mengirim token antara dua pengguna.
3. **Mint**: Mencetak token baru ke akun pengguna.
4. **Burn**: Membakar token dari akun pengguna.
5. **Query Balance**: Menampilkan saldo token untuk akun tertentu.

### Langkah-langkah Pembuatan Token Contract Lengkap di Solana

#### Langkah 1: Persiapkan Proyek

Jika Anda belum memiliki proyek Anchor, buat proyek baru dengan perintah berikut:
```bash
anchor init token_contract
cd token_contract
```

#### Langkah 2: Modifikasi File `lib.rs`

Buka dan edit file `lib.rs` di dalam folder `programs/token_contract/src/` seperti berikut:

```rust
use anchor_lang::prelude::*;

declare_id!("YourProgramID"); // Gantilah dengan ID program Anda

#[program]
pub mod token_contract {
    use super::*;

    // Inisialisasi akun token untuk pengguna
    pub fn initialize(ctx: Context<Initialize>, initial_supply: u64) -> Result<()> {
        let token_account = &mut ctx.accounts.token_account;
        token_account.supply = initial_supply;
        token_account.balance = initial_supply;
        Ok(())
    }

    // Transfer token antar akun
    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        let sender = &mut ctx.accounts.sender;
        let receiver = &mut ctx.accounts.receiver;

        if sender.balance < amount {
            return Err(ErrorCode::InsufficientBalance.into());
        }

        sender.balance -= amount;
        receiver.balance += amount;

        Ok(())
    }

    // Mint token baru ke akun
    pub fn mint(ctx: Context<Mint>, amount: u64) -> Result<()> {
        let token_account = &mut ctx.accounts.token_account;

        token_account.supply += amount;
        token_account.balance += amount;

        Ok(())
    }

    // Burn token dari akun
    pub fn burn(ctx: Context<Burn>, amount: u64) -> Result<()> {
        let token_account = &mut ctx.accounts.token_account;

        if token_account.balance < amount {
            return Err(ErrorCode::InsufficientBalance.into());
        }

        token_account.balance -= amount;
        token_account.supply -= amount;

        Ok(())
    }

    // Query saldo token
    pub fn query_balance(ctx: Context<QueryBalance>) -> Result<u64> {
        Ok(ctx.accounts.token_account.balance)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8 + 8)] // Alokasi ruang untuk supply dan saldo token
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub sender: Account<'info, TokenAccount>,
    #[account(mut)]
    pub receiver: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct Mint<'info> {
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct Burn<'info> {
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct QueryBalance<'info> {
    pub token_account: Account<'info, TokenAccount>,
}

#[account]
pub struct TokenAccount {
    pub balance: u64,  // Saldo token yang dimiliki oleh akun
    pub supply: u64,   // Total pasokan token
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient balance")]
    InsufficientBalance,
}
```

#### Langkah 3: Membuat Proyek dan Membangun Program

Setelah menulis kode di atas, kita perlu membangun dan mendeploy program. Jalankan perintah berikut:

1. **Membangun Program**:
   ```bash
   anchor build
   ```

2. **Mendeploy Program** ke jaringan Solana (misalnya testnet):
   ```bash
   solana config set --url https://api.devnet.solana.com
   anchor deploy
   ```

   Setelah deploy, Anda akan mendapatkan **Program ID** yang diperlukan untuk berinteraksi dengan kontrak.

#### Langkah 4: Interaksi dengan Program

Untuk berinteraksi dengan program, kita akan menggunakan **Anchor client** atau Solana CLI. Berikut adalah contoh penggunaan Anchor client untuk memanggil fungsi-fungsi dalam kontrak:

1. **Inisialisasi Akun Token**:
   ```bash
   anchor client invoke --program <ProgramID> --accounts token_account=<TokenAccountPubKey>,user=<UserPubKey> --data '{"initial_supply": 1000000}'
   ```

2. **Transfer Token**:
   ```bash
   anchor client invoke --program <ProgramID> --accounts sender=<SenderPubKey>,receiver=<ReceiverPubKey> --data '{"amount": 500}'
   ```

3. **Mint Token Baru**:
   ```bash
   anchor client invoke --program <ProgramID> --accounts token_account=<TokenAccountPubKey> --data '{"amount": 1000}'
   ```

4. **Burn Token**:
   ```bash
   anchor client invoke --program <ProgramID> --accounts token_account=<TokenAccountPubKey> --data '{"amount": 500}'
   ```

5. **Query Saldo Token**:
   ```bash
   anchor client invoke --program <ProgramID> --accounts token_account=<TokenAccountPubKey> --data '{}'
   ```

### Penjelasan Fungsi:
1. **Initialize**: Fungsi ini menginisialisasi akun token dengan pasokan awal (jumlah token yang dimiliki).
2. **Transfer**: Fungsi ini memungkinkan pengguna untuk mentransfer token ke pengguna lain, dengan memeriksa apakah pengirim memiliki cukup saldo.
3. **Mint**: Fungsi ini memungkinkan pencetakan token baru ke dalam akun pengguna.
4. **Burn**: Fungsi ini memungkinkan pengguna untuk membakar sejumlah token dari saldo mereka.
5. **QueryBalance**: Fungsi ini memungkinkan pengguna untuk mengecek saldo token mereka.

### Langkah 5: Menggunakan Solana CLI untuk Berinteraksi
Alternatif lainnya adalah menggunakan Solana CLI untuk melakukan interaksi seperti transfer dan minting, meskipun lebih mudah menggunakan Anchor client untuk mengelola akun dan program Solana secara efisien.

