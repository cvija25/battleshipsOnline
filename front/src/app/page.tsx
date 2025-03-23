// pages/index.tsx
import Head from "next/head";
import Link from "next/link";
import Login from "@/components/Login"
const Home: React.FC = () => {
  return (
    <div className="min-h-screen bg-gray-100 flex flex-col items-center justify-center">
      <Head>
        <title>Grid Component</title>
        <meta name="description" content="Grid with clickable cells" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className="p-4">
        <h1 className="text-2xl font-bold mb-6">Welcome to Battleships online!!</h1>
        <Login/>
        <Link href={'/game/board'}>play</Link>
      </main>
    </div>
  );
};

export default Home;
