import Sidebar from '../../components/userScreenInitial/sidebar';
import React from 'react';

const UserScreenInitial = () => {
  return (
    <div className="flex h-screen bg-[#E0E0E0]">
      <Sidebar />
      <main className="flex-1">
        {/* Main content will go here */}
      </main>
    </div>
  );
};

export default UserScreenInitial;
