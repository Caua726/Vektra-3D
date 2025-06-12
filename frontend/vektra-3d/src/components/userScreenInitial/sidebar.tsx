import React, { useState } from 'react';
import { ResizableBox } from 'react-resizable';
import 'react-resizable/css/styles.css';

const Sidebar = () => {
  const [width, setWidth] = useState(320); // 320px is w-80 in tailwind

  return (
    <ResizableBox
      width={width}
      height={Infinity}
      axis="x"
      minConstraints={[200, Infinity]}
      maxConstraints={[600, Infinity]}
      onResize={(e, data) => setWidth(data.size.width)}
      className="bg-[#212121] text-white flex flex-col h-screen shrink-0 relative"
      handle={<div className="absolute top-1/2 right-0 w-2 h-10 bg-gray-600 cursor-col-resize rounded" />}
    >
      <div className="p-4 overflow-y-auto">
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
    </ResizableBox>
  );
};

export default Sidebar;
