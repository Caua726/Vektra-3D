import React from 'react';

const PromptInput = () => {
  return (
    <div className="flex justify-center p-6">
      <input
        type="text"
        placeholder="Prompt"
        className="w-full max-w-xl bg-white border-2 border-black rounded-lg py-3 px-4 text-center font-semibold text-lg focus:outline-none"
      />
    </div>
  );
};

export default PromptInput; 