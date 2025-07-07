import { Suspense, useState } from 'react'
import './App.css'
import { Button, DataTable, type ColumnDef } from '@bc/ui'
import { Addresses } from './Addresses'
import React from 'react'
import { Balance } from './Balance'
import { SendBlock } from './SendBlock'

interface Recipe {
  id: string,
  title: string
}

export const columns: ColumnDef<Recipe>[] = [
  {
    accessorKey: 'id',
    header: 'Id'
  },
  {
    accessorKey: 'title',
    header: 'Title'
  }
]

const data: Recipe[] = [{
  id: '1',
  title: 'Abc title'
}]

function App() {
  const [count, setCount] = useState(0)

  return (
    <Suspense>
      <header className="bg-white shadow-md p-4 flex justify-between items-center">
        <h1 className="text-xl font-bold text-blue-600">MyCrypto Wallet</h1>
        <Addresses />
      </header>

      <main className="max-w-4xl mx-auto p-6 space-y-6">
        <section className="bg-white rounded-lg shadow p-6">
          <h2 className="text-lg font-semibold mb-2">Wallet Balance</h2>
          <Balance />
        </section>

        <section className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <SendBlock />

          <div className="bg-white rounded-lg shadow p-6">
            <h3 className="font-semibold text-lg mb-4">Receive</h3>
            <div className="bg-gray-100 p-4 rounded text-center">
              <div className="text-xs text-gray-600">Your Wallet Address</div>
              <div className="font-mono text-sm break-all">0xABC123DEF4567890</div>
              <button className="mt-2 text-sm text-blue-600 hover:underline">Copy Address</button>
            </div>
          </div>
        </section>

        <section className="bg-white rounded-lg shadow p-6">
          <h3 className="font-semibold text-lg mb-4">Recent Transactions</h3>
          <ul className="space-y-3">
            <li className="flex justify-between border-b pb-2">
              <div>
                <div className="font-medium">Sent 1.2 MYC</div>
                <div className="text-xs text-gray-500">To: 0x123...abc</div>
              </div>
              <div className="text-sm text-gray-500">2 hrs ago</div>
            </li>
            <li className="flex justify-between border-b pb-2">
              <div>
                <div className="font-medium">Received 0.5 MYC</div>
                <div className="text-xs text-gray-500">From: 0xdef...456</div>
              </div>
              <div className="text-sm text-gray-500">Yesterday</div>
            </li>
          </ul>
        </section>
      </main>
    </Suspense>
  )
}

export default App
