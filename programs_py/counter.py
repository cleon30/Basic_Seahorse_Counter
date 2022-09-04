# counter
# Built with Seahorse v0.1.6

from seahorse.prelude import *

declare_id('DyTz2YqoanAHMhDqUM9UwfzakXHoGdueLogNnbLTkceL')

class Counter(Account):
  owner: Pubkey
  count: i64

@instruction
def init_counter(owner: Signer, counter: Empty[Counter]):
    counter = counter.init(payer =  owner, seeds = ['Counter', owner])
    counter.owner = owner.key()

@instruction
def add(owner: Signer, counter: Counter):
    assert owner.key() == counter.owner, 'This is not your counter!'

    counter.count +=1

