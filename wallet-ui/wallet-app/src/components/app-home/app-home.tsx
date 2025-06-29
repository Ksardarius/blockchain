import { Component, h, State } from '@stencil/core';
import { Router } from "../../";
import init, {generate_new_key_pair, create_wallet, get_all_wallets, create_transaction} from 'wallet-web-wasm'

@Component({
  tag: 'app-home',
  styleUrl: 'app-home.css',
  shadow: true,
})
export class AppHome {
  @State()
  selected_address: string = ''

  @State()
  addresses: string[] = []

  componentWillLoad() {
    return init();
  }

  async generate_wallet() {
    await create_wallet("123");
  }

  async get_all_wallets() {
    this.addresses = await get_all_wallets();
    this.selected_address = this.addresses[0];
    console.log(this.addresses)
  }

  async create_new_transaction() {
    let tx = await create_transaction(this.selected_address, "123", "ef637f6b2a16df8eec2b986e9df5383b9ed48514", BigInt(100));
    console.log('Transaction created', tx)
  }

  handle_select(event) {
    this.selected_address = event.target.value
  }

  async handle_submit() {

  }

  render() {
    const key = generate_new_key_pair()
    return (
      <div class="app-home">
        <span>{key}</span>
        <div>
          <form onSubmit={(e) => this.handle_submit()}>
            <label>
              Address:
              <select onInput={(event) => this.handle_select(event)}>
                {
                  this.addresses.map(a => {
                    return <option value={a} selected={a === this.selected_address}>{a}</option>
                  })
                }
              </select>
            </label>
            <input type="submit" value="Submit" />
          </form>
        </div>
        <p>
          Welcome to the Stencil App Starter. You can use this starter to build entire apps all with web components using Stencil! Check out our docs on{' '}
          <a href="https://stenciljs.com">stenciljs.com</a> to get started.
        </p>
        <button
          onClick={() => { this.generate_wallet() }}
        >
          Generate Key
        </button>
        <button
          onClick={() => { this.get_all_wallets() }}
        >
          Get All Wallets
        </button>
        <button
          onClick={() => { this.create_new_transaction() }}
        >
          Create new transactions
        </button>
        <button
          onClick={() => Router.push('/profile/stencil')}
        >
          Profile Page
        </button>
      </div>
    );
  }
}
