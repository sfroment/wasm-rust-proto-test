import * as wasm from "proto-client";
import { Test } from "@/components/test";

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <Test />
    </main>
  );
}
