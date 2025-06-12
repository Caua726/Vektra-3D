import Sidebar from '../../components/userScreenInitial/sidebar';
import React from 'react';
import Header from '../../components/userScreenInitial/Header';
import PromptInput from '../../components/userScreenInitial/PromptInput';

const UserScreenInitial = () => {
  return (
    <div className="flex h-screen bg-[#E0E0E0]">
      <Sidebar />
      <main className="flex-1 flex flex-col">
        <Header />
        <div className="flex-1 flex flex-col justify-between">
            <div className="flex-1">
                {/* Main content will go here */}
            </div>
            <PromptInput />
        </div>
      </main>
    </div>
  );
};

export default UserScreenInitial;
