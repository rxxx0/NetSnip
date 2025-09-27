import React, { useState, useRef } from 'react';
import { Settings, User, Shield, Bell, Database, HelpCircle, LogOut } from 'lucide-react';
import { useClickOutside } from '../../hooks/useClickOutside';

interface MenuItemProps {
  icon: React.ReactNode;
  label: string;
  onClick?: () => void;
}

const MenuItem: React.FC<MenuItemProps> = ({ icon, label, onClick }) => (
  <button
    onClick={onClick}
    className="w-full flex items-center gap-3 px-3 py-2 text-sm text-text-primary hover:bg-gray-100 dark:hover:bg-gray-800 rounded transition-colors"
  >
    <span className="w-4 h-4 flex items-center justify-center text-gray-400">{icon}</span>
    <span>{label}</span>
  </button>
);

const Separator: React.FC = () => (
  <div className="mt-1 mb-1 h-px bg-border-light" />
);

export const SettingsMenu: React.FC = () => {
  const [isOpen, setIsOpen] = useState(false);
  const menuRef = useRef<HTMLDivElement>(null);

  useClickOutside(menuRef, () => setIsOpen(false), isOpen);

  return (
    <div className="relative" ref={menuRef}>
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="neu-button p-2 rounded-lg"
        aria-label="Settings"
      >
        <Settings className="w-4 h-4" />
      </button>

      {isOpen && (
        <div className="absolute right-0 mt-2 w-56 neu-card rounded-lg p-1 animate-slide-down z-50">
          <MenuItem icon={<User />} label="Account" />
          <MenuItem icon={<Shield />} label="Security" />
          <MenuItem icon={<Bell />} label="Notifications" />
          <MenuItem icon={<Database />} label="Data & Privacy" />
          <MenuItem icon={<HelpCircle />} label="Help & Support" />
          <Separator />
          <MenuItem icon={<LogOut />} label="Sign Out" />
        </div>
      )}
    </div>
  );
};