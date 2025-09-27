import React, { createContext, useContext, useState, useCallback, type ReactNode } from 'react';
import { NotificationModal, type NotificationType } from '../components/common/NotificationModal';

interface NotificationOptions {
  type: NotificationType;
  title: string;
  message: string;
  confirmText?: string;
  cancelText?: string;
}

interface NotificationContextType {
  showNotification: (options: NotificationOptions) => Promise<boolean>;
}

const NotificationContext = createContext<NotificationContextType | null>(null);

export const useNotification = () => {
  const context = useContext(NotificationContext);
  if (!context) {
    throw new Error('useNotification must be used within NotificationProvider');
  }
  return context;
};

export const NotificationProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [isOpen, setIsOpen] = useState(false);
  const [options, setOptions] = useState<NotificationOptions>({
    type: 'alert',
    title: '',
    message: '',
  });
  const [resolvePromise, setResolvePromise] = useState<((value: boolean) => void) | null>(null);

  const showNotification = useCallback((opts: NotificationOptions): Promise<boolean> => {
    return new Promise((resolve) => {
      setOptions(opts);
      setIsOpen(true);
      setResolvePromise(() => resolve);
    });
  }, []);

  const handleConfirm = useCallback(() => {
    if (resolvePromise) {
      resolvePromise(true);
      setResolvePromise(null);
    }
    setIsOpen(false);
  }, [resolvePromise]);

  const handleCancel = useCallback(() => {
    if (resolvePromise) {
      resolvePromise(false);
      setResolvePromise(null);
    }
    setIsOpen(false);
  }, [resolvePromise]);

  const handleClose = useCallback(() => {
    if (resolvePromise) {
      resolvePromise(options.type !== 'confirm');
      setResolvePromise(null);
    }
    setIsOpen(false);
  }, [resolvePromise, options.type]);

  return (
    <NotificationContext.Provider value={{ showNotification }}>
      {children}
      <NotificationModal
        isOpen={isOpen}
        type={options.type}
        title={options.title}
        message={options.message}
        confirmText={options.confirmText}
        cancelText={options.cancelText}
        onConfirm={handleConfirm}
        onCancel={handleCancel}
        onClose={handleClose}
      />
    </NotificationContext.Provider>
  );
};