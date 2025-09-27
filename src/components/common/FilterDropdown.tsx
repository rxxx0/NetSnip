import React, { useState, useRef } from 'react';
import { Filter, Check } from 'lucide-react';
import { useClickOutside } from '../../hooks/useClickOutside';

interface FilterOption {
  value: 'all' | 'online' | 'blocked' | 'limited';
  label: string;
}

interface FilterDropdownProps {
  value: 'all' | 'online' | 'blocked' | 'limited';
  onChange: (value: 'all' | 'online' | 'blocked' | 'limited') => void;
}

const options: FilterOption[] = [
  { value: 'all', label: 'All Devices' },
  { value: 'online', label: 'Online' },
  { value: 'blocked', label: 'Blocked' },
  { value: 'limited', label: 'Limited' },
];

export const FilterDropdown: React.FC<FilterDropdownProps> = ({ value, onChange }) => {
  const [isOpen, setIsOpen] = useState(false);
  const dropdownRef = useRef<HTMLDivElement>(null);

  useClickOutside(dropdownRef, () => setIsOpen(false), isOpen);

  const selectedOption = options.find(opt => opt.value === value);

  return (
    <div className="relative" ref={dropdownRef}>
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="neu-button px-4 py-2 text-sm flex items-center gap-2 min-w-[140px]"
      >
        <Filter className="w-4 h-4 text-gray-400" />
        <span className="flex-1 text-left">{selectedOption?.label}</span>
        <svg
          className={`w-3 h-3 transition-transform ${isOpen ? 'rotate-180' : ''}`}
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
        </svg>
      </button>

      {isOpen && (
        <div className="absolute right-0 mt-1 w-full min-w-[140px] neu-card rounded-lg p-1 animate-slide-down z-50">
          {options.map(option => (
            <button
              key={option.value}
              onClick={() => {
                onChange(option.value);
                setIsOpen(false);
              }}
              className="w-full flex items-center justify-between px-3 py-2 text-sm text-text-primary hover:bg-gray-100 dark:hover:bg-gray-800 rounded transition-colors"
            >
              <span>{option.label}</span>
              {value === option.value && (
                <Check className="w-3 h-3 text-green-500" />
              )}
            </button>
          ))}
        </div>
      )}
    </div>
  );
};