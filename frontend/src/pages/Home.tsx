import WalletBar from "../components/WalletBar";
import contractAbi from "../abi.json";
import {
  useAccount,
  useContract,
  useContractRead,
  useContractWrite,
} from "@starknet-react/core";
import { useEffect, useMemo, useState } from "react";
import { Button } from "../components/Button";
import { zeroPadHex } from "../utils/utils";

function Home() {
  const BASE_BACKEND_URL = "http://35.195.237.203:8080/";
  const CONTRACT_ADDRESS =
    "0x03e942530ef96da8e65e453f0fbbb198994515c69edd1dcf3be353b0956fbd1a";

  const { contract } = useContract({
    abi: contractAbi,
    address: CONTRACT_ADDRESS,
  });
  const { address } = useAccount();

  // Data to be utilized for claiming tokens
  interface ClaimCalldata {
    // How much to claim. Should always claim the maximum amount
    amount: string;
    // Merkle proof for the claim
    proof: string[];
  }

  const [walletAddress, setWalletAddress] = useState<string>("");
  const [alreadyClaimed, setAlreadyClaimed] = useState<bigint>(0n);
  const [allocationAmount, setAllocationAmount] = useState<bigint>(0n);
  const [receivedcalldata, setReceivedCalldata] = useState<ClaimCalldata>();
  const [isClaimReady, setIsClaimReady] = useState<boolean>(false);
  const [errors, setErrors] = useState<string>("");

  const {
    data: alreadyClaimedData,
    isError,
    isLoading,
  } = useContractRead({
    functionName: "amount_already_claimed",
    args: [walletAddress!],
    abi: contractAbi,
    address: CONTRACT_ADDRESS,
    watch: false,
  });

  useEffect(() => {
    if (address) {
      const addr = zeroPadHex(address!);
      setWalletAddress(addr);
      prepareClaim(addr);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [address]);

  useEffect(() => {
    if (!isLoading && !isError) {
      setAlreadyClaimed(alreadyClaimedData as bigint);
    }
  }, [isLoading, isError, alreadyClaimedData]);

  useEffect(() => {
    const getAllocation = async () => {
      const response = await fetch(
        BASE_BACKEND_URL + "get_allocation_amount?address=" + walletAddress
      );
      const amount = await response.json();
      const num = BigInt(amount);

      setAllocationAmount(num);
    };
    if (walletAddress) {
      getAllocation();
    }
  }, [walletAddress]);

  const calls = useMemo(() => {
    if (!walletAddress || !contract || !receivedcalldata) return [];

    if (!receivedcalldata.amount) {
      setErrors(receivedcalldata.toString());
      return;
    }

    return contract.populateTransaction["claim"]!(
      receivedcalldata.amount,
      receivedcalldata.proof
    );
  }, [contract, receivedcalldata, walletAddress]);

  const { writeAsync: callClaim } = useContractWrite({
    calls,
  });

  // Retrieves calldata for the claim
  const prepareClaim = async (usedAddress: string) => {
    if (!usedAddress) {
      console.error("No wallet connected");
      return;
    }

    const response = await fetch(
      BASE_BACKEND_URL + "get_calldata?address=" + usedAddress
    );
    const calldata: ClaimCalldata = await response.json();

    setReceivedCalldata(calldata);

    setIsClaimReady(true);
  };

  const claim = async () => {
    if (!isClaimReady) {
      console.error("Prepare the claim first");
      return;
    }
    await callClaim();
  };

  return (
    <main className="flex flex-col items-center justify-center min-h-screen gap-12">
      <WalletBar />
      {isClaimReady && (
        <div>
          <div>
            <Button onClick={claim}>Claim allocation</Button>
          </div>
        </div>
      )}
      <div>
        {alreadyClaimed !== undefined && (
          <div>Already claimed: {alreadyClaimed.toString()}</div>
        )}
        <div>
          <p>Total allocated amount: {allocationAmount.toString()}</p>
        </div>
        {errors && (
          <div>
            <p style={{ color: "red" }}>Errors: {errors}</p>
          </div>
        )}
      </div>
    </main>
  );
}

export default Home;
