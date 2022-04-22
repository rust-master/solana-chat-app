import React from 'react';
import logo from './logo.svg';
import './App.css';

import {
  Keypair,
  Connection,
  PublicKey,
  LAMPORTS_PER_SOL,
  SystemProgram,
  TransactionInstruction,
  Transaction,
  sendAndConfirmTransaction,
} from '@solana/web3.js';
import * as solanaWeb3 from '@solana/web3.js';


import fs from 'mz/fs';
import path from 'path';
import * as borsh from 'borsh';


const PROGRAM_PATH = path.resolve(__dirname, '../program/target/deploy');
const PROGRAM_SO_PATH = path.join(PROGRAM_PATH, 'program.so');
const PROGRAM_KEYPAIR_PATH = path.join(PROGRAM_PATH, 'program-keypair.json');


class App extends React.Component {

  constructor(props) {
    super(props);
    this.state = {
      connection: Connection,
      payer: Keypair,
      programId: PublicKey,
      greetedPubkey: PublicKey,
    }
  }

  async establishConnection() {
    const rpcUrl = solanaWeb3.clusterApiUrl();
    connection = new Connection(rpcUrl, 'confirmed');
    const version = await this.state.connection.getVersion();
    console.log('Connection to cluster established:', rpcUrl, version);
  }


  render() {
    return (
      <div className="App">
        <header className="App-header">
          <img src={logo} className="App-logo" alt="logo" />
          <p>
            Edit <code>src/App.js</code> and save to reload.
          </p>
          <a
            className="App-link"
            href="https://reactjs.org"
            target="_blank"
            rel="noopener noreferrer"
          >
            Learn React
          </a>
        </header>
      </div>
    );
  }
}

export default App;
