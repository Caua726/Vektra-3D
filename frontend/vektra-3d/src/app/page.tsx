import { LayoutTemplate, Folder, Image, FilePlus, Send, LucideProps } from "lucide-react";
import { FC, ElementType } from 'react';

const VektraLogo = () => (
    <div className="flex items-center gap-3">
      <img src="/vektra_logo.png" alt="Vektra Logo" className="h-16 w-16" />
      <span className="text-2xl font-bold tracking-wider">Vektra-3D</span>
    </div>
);

interface ActionButtonProps {
  icon: ElementType<LucideProps>;
  text: string;
}

const ActionButton: FC<ActionButtonProps> = ({ icon: Icon, text }) => {
  return (
    <button className="w-full text-left flex items-center gap-4 p-4 rounded-xl bg-gradient-to-br from-[#302742] to-[#211e2b] hover:from-[#3c3153] hover:to-[#2b283b] transition-all duration-300 border border-white/10 shadow-lg shadow-black/30 transform hover:-translate-y-1">
      <Icon className="h-6 w-6 text-purple-300" />
      <span className="text-gray-200 font-medium">{text}</span>
    </button>
  );
};

export default function Home() {
  return (
    <div className="bg-[#100F12] text-white min-h-screen flex flex-col items-center justify-center font-sans overflow-hidden">
      {/* Header */}
      <header className="absolute top-0 left-0 w-full p-8 flex justify-center items-center z-10">
        <VektraLogo />
      </header>

      <div className="flex w-full max-w-screen-xl mx-auto items-center justify-center gap-12 px-4">
        <aside className="w-[340px] space-y-5">
          <ActionButton icon={LayoutTemplate} text="Comece com um template." />
          <ActionButton icon={Folder} text="Abrir projeto." />
          <ActionButton icon={Image} text="Comece com uma imagem." />
          <ActionButton icon={FilePlus} text="Comece com um modelo vazio." />
        </aside>

        <main className="w-[500px] flex items-center justify-center py-20">
            <div className="relative group">
                <div
                  className="absolute -inset-4 bg-gradient-to-br from-purple-700 to-indigo-600 rounded-3xl blur-3xl opacity-80 group-hover:opacity-100 transition-all duration-700 animate-pulseBig"
                  style={{
                    animation: 'pulseBig 2.5s cubic-bezier(0.4, 0, 0.6, 1) infinite',
                  }}
                ></div>
                <div className="relative bg-black/40 backdrop-blur-xl p-16 rounded-3xl text-center border border-white/10 shadow-2xl shadow-purple-900/40">
                    <h1 className="text-5xl font-bold mb-3 bg-gradient-to-b from-gray-100 to-gray-400 text-transparent bg-clip-text">Bem-vindo ao Vektra!</h1>
                    <p className="text-gray-400 mb-10 text-lg">O que vamos criar hoje?</p>
                    <button className="flex items-center justify-center gap-2.5 mx-auto bg-gradient-to-br from-blue-500 to-blue-600 hover:from-blue-400 hover:to-blue-600 text-white font-semibold py-3 px-7 rounded-lg transition-all duration-300 shadow-xl shadow-blue-500/30 transform hover:scale-105 focus:outline-none focus:ring-4 focus:ring-blue-500/50">
                        <Send className="h-5 w-5" />
                        <span>Fazer um tour</span>
                    </button>
                </div>
            </div>
        </main>
      </div>
    </div>
  );
}
