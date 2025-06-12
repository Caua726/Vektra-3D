import React from 'react';

const Header = () => {
  return (
    <header className="flex justify-between items-center px-4 border-b-2 border-gray-300 bg-[#FFF]">
      <nav className="flex items-end -mb-0.5">
        <button className="px-4 py-2 text-sm font-semibold text-white bg-[#6E41E2] border-b-2 border-white">
          Modelagem
        </button>
        <button className="px-4 py-2 ml-4 text-sm font-semibold text-black">
          Animação
        </button>
        <button className="px-4 py-2 ml-4 text-sm font-semibold text-black">
          Preview
        </button>
        <button className="px-4 py-2 ml-4 text-sm font-semibold text-black">
          AI
        </button>
      </nav>
      <button className="px-5 py-1 text-sm font-semibold bg-white border-2 border-black rounded-lg">
        Exportar
      </button>
    </header>
  );
};

export default Header; 