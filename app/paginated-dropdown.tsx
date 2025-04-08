import React, { useState, useRef, useEffect } from 'react';
import { topic } from './types';

interface Props {
  options: topic[];
  onChange: (selectedOption: topic | null) => void;
  placeholder?: string;
  itemsPerPage?: number;
}

function PaginatedDropdown({ options, onChange, placeholder = 'Topic', itemsPerPage = 6 }: Props) {
  const [isOpen, setIsOpen] = useState<boolean>(false);
  const [currentPage, setCurrentPage] = useState<number>(0);
  const dropdownRef = useRef<HTMLDivElement>(null);
  const [selectedOption, setSelectedOption] = useState<topic | null>(null);

  const totalPages = Math.ceil(options.length / itemsPerPage);
  const startIndex = currentPage * itemsPerPage;
  const endIndex = startIndex + itemsPerPage;
  const currentOptions = options.slice(startIndex, endIndex);

  const toggleDropdown = (): void => {
    setIsOpen((prev) => !prev);
  };

  const handleOptionClick = (option: topic): void => {
    setSelectedOption(option);
    onChange(option);
    setIsOpen(false);
  };

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    };

    document.addEventListener('mousedown', handleClickOutside);
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, [dropdownRef]);


  const handlePrevPage = (): void => {
    setCurrentPage((prev) => Math.max(prev - 1, 0));
  };

  const handleNextPage = (): void => {
    setCurrentPage((prev) => Math.min(prev + 1, totalPages - 1));
  };

  return (
    <div className="relative inline-block text-left border-transparent border-b-neutral-500 border-2 -ml-0.5" ref={dropdownRef}>
      <div>
        <button
          type="button"
          className="inline-flex justify-between items-center w-full rounded-t-md border border-black shadow-sm px-4 py-2 bg-neutral-600 text-sm font-medium text-neutral-200 hover:bg-neutral-800 focus:outline-none focus:ring-2 focus:ring-[var(--accent)]"
          id="menu-button"
          aria-expanded={isOpen ? "true" : "false"}
          aria-haspopup="true"
          onClick={toggleDropdown}
        >
          {selectedOption ? selectedOption.name : placeholder}
          <svg className="-mr-1 ml-2 h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
            <path fillRule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clipRule="evenodd" />
          </svg>
        </button>
      </div>

      <div
        className={`origin-bottom-right absolute bottom-full right-0 mb-2 w-56 rounded-md shadow-lg bg-neutral-500 ring-1 ring-black ring-opacity-5 focus:outline-none z-10 ${
          isOpen ? '' : 'hidden'
        }`}
        role="menu"
        aria-orientation="vertical"
        aria-labelledby="menu-button"
        tabIndex={-1}
      >
        <div className="py-1 max-h-60 overflow-y-auto" role="none">
          {currentOptions.map((option) => (
            <div
              key={option.id}
              onClick={() => handleOptionClick(option)}
              className="text-neutral-200 block px-4 py-2 text-sm hover:bg-neutral-800 cursor-pointer"
              role="menuitem"
              tabIndex={-1}
              id={`menu-item-${option.id}`}
            >
              {option.name}
            </div>
          ))}
        </div>

        {options.length === 0 && (
          <div>
            No Topics have been added.
          </div>
        )}

        {totalPages > 1 && (
          <div className="flex justify-between items-center px-2 py-1">
            <button
              onClick={handlePrevPage}
              disabled={currentPage === 0}
              className="px-2 py-1 rounded bg-neutral-600 hover:bg-neutral-800 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              Prev
            </button>
            <span>{currentPage + 1} / {totalPages}</span>
            <button
              onClick={handleNextPage}
              disabled={currentPage === totalPages - 1}
              className="px-2 py-1 rounded bg-neutral-600 hover:bg-neutral-800 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              Next
            </button>
          </div>
        )}
      </div>
    </div>
  );
}

export default PaginatedDropdown;
