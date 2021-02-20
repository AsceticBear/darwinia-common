const expect = require("chai").expect;
const assert = require("chai").assert;
const Web3 = require("web3");
const conf = require("./config.js");
const web3 = new Web3(conf.host);
const account = web3.eth.accounts.wallet.add(conf.privKey);
const bytecode =
	"608060405234801561001057600080fd5b50610134806100206000396000f3fe608060405260043610601c5760003560e01c80633ccfd60b146021575b600080fd5b60276029565b005b602f60ba565b603560dc565b7f723908ee9dc8e509d4b93251bd57f68c09bd9d04471c193fabd8f26c54284a4b81600060018110608f577f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b6020020181815250506040826020836801a055690d9db800006005600019f160b657600080fd5b5050565b6040518060600160405280600390602082028036833780820191505090505090565b604051806020016040528060019060208202803683378082019150509050509056fea2646970667358221220ab9bbb427c10e58a0d7a602af6becca3384a7158987eea7093ad32b895c7439264736f6c63430008010033";
const abi = [
	{
		inputs: [],
		name: "withdraw",
		outputs: [],
		stateMutability: "payable",
		type: "function",
	},
];
const jsontest = new web3.eth.Contract(abi);
jsontest.options.from = conf.address;
jsontest.options.gas = conf.gas;

describe("Test Transfer Contract", function () {
	after(() => {
		web3.currentProvider.disconnect();
	});
	it("Deploy transfer back contract", async function () {
		const instance = await jsontest
			.deploy({
				data: bytecode,
				arguments: [],
			})
			.send();
		jsontest.options.address = instance.options.address;
		console.log("contract address {}", jsontest.options.address);
	}).timeout(10000);
	it("Transfer back", async function () {
		await jsontest.methods.withdraw().send({
			value: 10000000000000000000,
		});
	}).timeout(80000);
});