import React from 'react';

const Sidebar = () => {
  return (
    <aside className="w-100 bg-[#212121] text-white flex flex-col h-screen shrink-0">
      <div className="p-4">
        <nav className="border-b border-gray-700">
          <ul className="flex items-center space-x-4 text-sm">
            <li className="pb-2 border-b-2 border-white">
              <button className="font-semibold text-white">Histórico</button>
            </li>
            <li className="pb-2">
              <button className="text-gray-400 hover:text-white">Propriedades</button>
            </li>
            <li className="pb-2">
              <button className="text-gray-400 hover:text-white">Templates</button>
            </li>
            <li className="pb-2">
              <button className="text-gray-400 hover:text-white">Biblioteca</button>
            </li>
          </ul>
        </nav>
        <div className="mt-4">
          <ul>
            <li className="py-3 border-b border-gray-700">
              <p className="text-sm truncate">Lorem ipsum dolor sit amet consectetur adipisicing elit. Quisquam, quos.</p>
            </li>
            <li className="py-3 border-b border-gray-700">
              <p className="text-sm truncate">Lorem ipsum dolor sit amet consectetur adipisicing elit. Quisquam, quos.</p>
            </li>
          </ul>
        </div>
      </div>
    </aside>
  );
};

export default Sidebar;
