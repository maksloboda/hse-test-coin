const { expect } = require("chai");
const { loadFixture } = require("@nomicfoundation/hardhat-network-helpers");


describe("AmogusToken contract", function () {
  async function deployTokenFixture() {
    const Token = await ethers.getContractFactory("AmogusToken");
    const [owner, addr1, addr2] = await ethers.getSigners();

    const amogusToken = await Token.deploy();

    await amogusToken.deployed();
    
    return { amogusToken, owner, addr1, addr2 };
  }
  
  var ownerBalance

  it("Correct default values", async function () {
    const { amogusToken, owner, addr1, addr2 } = await loadFixture(
      deployTokenFixture
    );

    
    // Owner should have all the tokens
    ownerBalance = await amogusToken.balanceOf(owner.address);
    expect(await amogusToken.totalSupply()).to.equal(ownerBalance);
    const addr1Balance = await amogusToken.balanceOf(addr1.address);
    expect(0).to.equal(addr1Balance);
    const addr2Balance = await amogusToken.balanceOf(addr2.address);
    expect(0).to.equal(addr2Balance);
    // Default allowance should be 0
    expect(await amogusToken.allowance(owner.address, addr1.address)).to.equal(0);
  })

  it("Transfering tokens", async function () {
    const { amogusToken, owner, addr1, addr2 } = await loadFixture(
      deployTokenFixture
    );

    await expect(amogusToken.connect(addr1).transfer(addr2.address, 1)).to.be.revertedWith("Not enough tokens");
    await expect(amogusToken.connect(owner).transfer(addr2.address, ownerBalance + 1)).to.be.revertedWith("Not enough tokens");
    
    const transferAmount = 20;
    await expect(amogusToken.connect(owner).transfer(addr1.address, transferAmount)).to
    .changeTokenBalances(amogusToken, [owner, addr1], [-transferAmount, transferAmount]);

    await expect(amogusToken.connect(owner).transfer(addr1.address, transferAmount)).to
    .emit(amogusToken, "Transfer").withArgs(owner.address, addr1.address, transferAmount)

    const aproveAmount = 20;
    await expect(amogusToken.connect(addr1).approve(addr2.address, aproveAmount)).to
    .emit(amogusToken, "Approval").withArgs(addr1.address, addr2.address, aproveAmount)

    expect(await amogusToken.allowance(addr1.address, addr2.address)).to.equal(aproveAmount)
    expect(await amogusToken.allowance(addr2.address, addr1.address)).to.equal(0)

    await expect(amogusToken.connect(addr2).transferFrom(addr1.address, owner.address, aproveAmount - 1)).to
    .emit(amogusToken, "Transfer").withArgs(addr1.address, owner.address, aproveAmount - 1)

    await expect(amogusToken.connect(addr2).transferFrom(addr1.address, owner.address, 1)).to
    .changeTokenBalances(amogusToken, [owner, addr1], [1, -1]);

    await expect(amogusToken.connect(addr2).transferFrom(addr1.address, owner.address, 1)).to
    .be.revertedWith("Not enough allowance");

  })

});
