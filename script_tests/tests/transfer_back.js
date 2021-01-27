const expect = require("chai").expect;
const Web3 = require("web3");

const web3 = new Web3("http://localhost:9933");

const addressFrom = "0x6Be02d1d3665660d22FF9624b7BE0551ee1Ac91b";
// substrate: '5ELRpquT7C3mWtjeqFMYqgNbcNgWKSr3mYtVi1Uvtc2R7YEx';
const privKey = "99B3C12287537E38C90A9219D4CB074A89A16E9CDB20BF85728EBD97C343E342";

const addressTo = "0x0000000000000000000000000000000000000005";

describe("Test transfer back", function () {
    it("Transfer back", async function() {
        const input = "723908ee9dc8e509d4b93251bd57f68c09bd9d04471c193fabd8f26c54284a4b005ed0b200000000000000000000000000000000000000000000000000000000";

        const createTransaction = await web3.eth.accounts.signTransaction(
			{
				from: addressFrom,
				to: addressTo,
				value: web3.utils.toWei("3", "ether"),
                gas: "5000000000",
                data: input,
			},
			privKey
        );

        const createReceipt = await web3.eth.sendSignedTransaction(
            createTransaction.rawTransaction
        );
    }).timeout(10000);
});

0x1025.transfer(msg.sender /*to*/, wad)