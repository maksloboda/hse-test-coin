//SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

contract AmogusToken {
    string public name = "AmogusToken";
    string public symbol = "AMOT";

    address public immutable owner;
    uint256 public totalSupply = 1000;

    struct BalanceData {
        uint256 balance;
        mapping(address => uint256) allowance; 
    }

    mapping(address => BalanceData) balances;

    event Transfer(address indexed _from, address indexed _to, uint256 _value);
    event Approval(address indexed _owner, address indexed _spender, uint256 _value);

    constructor() {
        balances[msg.sender].balance = totalSupply;
        owner = msg.sender;
    }

    function decimals() public pure returns (uint8) {
        return 0;
    }

    function balanceOf(address _owner) public view returns (uint256 balance) {
        return balances[_owner].balance;
    }

    function rawTransfer(address _from, address _to, uint256 _value) internal returns (bool success) {
        require(balances[_from].balance >= _value, "Not enough tokens");
        balances[_from].balance -= _value;
        balances[_to].balance += _value;
        emit Transfer(_from, _to, _value);
        return true;
    }

    function transfer(address _to, uint256 _value) public returns (bool success) {
        return rawTransfer(msg.sender, _to, _value);
    }

    function transferFrom(address _from, address _to, uint256 _value) public returns (bool success) {
        require(balances[_from].allowance[msg.sender] >= _value, "Not enough allowance");
        bool ok = rawTransfer(_from, _to, _value);
        if (ok) {
            balances[_from].allowance[msg.sender] -= _value;
        }
        return ok;
    }

    function approve(address _spender, uint256 _value) public returns (bool success) {
        balances[msg.sender].allowance[_spender] = _value;
        emit Approval(msg.sender, _spender, _value);
        return true;
    }

    function allowance(address _owner, address _spender) public view returns (uint256 remaining) {
        return balances[_owner].allowance[_spender];
    }

}
