from iota_sdk import Wallet

# In this example we will get outputs stored in the account

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

# All outputs stored in the account
outputs = account.outputs()
print(f'Outputs: {outputs}')

# Only the unspent outputs in the account
unspent_outputs = account.outputs()
print(f'Unspent outputs: {unspent_outputs}')
