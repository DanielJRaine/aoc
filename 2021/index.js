#! /usr/bin/env node
const { program } = require('commander')
const part = require('./commands/part')

program
    .command('part <id>')
    .description('Solve part')
    .action(part)

program.parse();
