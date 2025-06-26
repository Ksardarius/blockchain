import { Component, h } from '@stencil/core';
import { Router } from "../../";
import init, {generate_new_key_pair} from 'wallet-web-wasm'

@Component({
  tag: 'app-home',
  styleUrl: 'app-home.css',
  shadow: true,
})
export class AppHome {
  componentWillLoad() {
    return init();
  }

  render() {
    const key = generate_new_key_pair()
    return (
      <div class="app-home">
        <span>{key}</span>
        <p>
          Welcome to the Stencil App Starter. You can use this starter to build entire apps all with web components using Stencil! Check out our docs on{' '}
          <a href="https://stenciljs.com">stenciljs.com</a> to get started.
        </p>
        <button
          onClick={() => Router.push('/profile/stencil')}
        >
          Profile Page
        </button>
      </div>
    );
  }
}
