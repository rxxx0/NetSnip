import React, { useEffect } from 'react';
import { AlertTriangle, Info, CheckCircle, X } from 'lucide-react';

export type NotificationType = 'confirm' | 'alert' | 'success' | 'error' | 'warning';

interface NotificationModalProps {
  isOpen: boolean;
  type: NotificationType;
  title: string;
  message: string;
  confirmText?: string;
  cancelText?: string;
  onConfirm?: () => void;
  onCancel?: () => void;
  onClose: () => void;
}

export const NotificationModal: React.FC<NotificationModalProps> = ({
  isOpen,
  type,
  title,
  message,
  confirmText = 'Confirm',
  cancelText = 'Cancel',
  onConfirm,
  onCancel,
  onClose,
}) => {
  // Close on Escape key
  useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && isOpen) {
        if (type === 'confirm' && onCancel) {
          onCancel();
        } else {
          onClose();
        }
      }
    };

    if (isOpen) {
      document.addEventListener('keydown', handleEscape);
      // Prevent body scroll when modal is open
      document.body.style.overflow = 'hidden';
    }

    return () => {
      document.removeEventListener('keydown', handleEscape);
      document.body.style.overflow = 'unset';
    };
  }, [isOpen, type, onCancel, onClose]);

  if (!isOpen) return null;

  const getIcon = () => {
    switch (type) {
      case 'warning':
      case 'confirm':
        return <AlertTriangle className="w-6 h-6 text-yellow-500" />;
      case 'error':
        return <X className="w-6 h-6 text-red-500" />;
      case 'success':
        return <CheckCircle className="w-6 h-6 text-green-500" />;
      default:
        return <Info className="w-6 h-6 text-blue-500" />;
    }
  };

  const getAccentColor = () => {
    switch (type) {
      case 'warning':
      case 'confirm':
        return 'var(--accent-warning)';
      case 'error':
        return 'var(--accent-danger)';
      case 'success':
        return 'var(--accent-success)';
      default:
        return 'var(--accent-primary)';
    }
  };

  return (
    <>
      {/* Backdrop */}
      <div
        className="fixed inset-0 z-[9998] bg-black/50 backdrop-blur-sm animate-fade-in"
        style={{ animationDuration: '200ms' }}
        onClick={type !== 'confirm' ? onClose : undefined}
      />

      {/* Modal */}
      <div className="fixed inset-0 z-[9999] flex items-center justify-center p-4">
        <div
          className="neu-card max-w-md w-full p-6 animate-scale-in relative"
          style={{
            animationDuration: '300ms',
            boxShadow: '0 20px 40px rgba(0, 0, 0, 0.3)',
          }}
        >
          {/* Icon and Title */}
          <div className="flex items-center gap-3 mb-4">
            <div
              className="w-12 h-12 rounded-full flex items-center justify-center"
              style={{
                background: `${getAccentColor()}20`,
              }}
            >
              {getIcon()}
            </div>
            <h3 className="text-lg font-semibold text-text-primary flex-1">
              {title}
            </h3>
          </div>

          {/* Message */}
          <p className="text-sm text-text-secondary mb-6 leading-relaxed">
            {message}
          </p>

          {/* Actions */}
          <div className="flex gap-3 justify-end">
            {type === 'confirm' && (
              <button
                onClick={() => {
                  if (onCancel) onCancel();
                  onClose();
                }}
                className="neu-button px-4 py-2 text-sm"
              >
                {cancelText}
              </button>
            )}
            <button
              onClick={() => {
                if (type === 'confirm' && onConfirm) {
                  onConfirm();
                }
                onClose();
              }}
              className="neu-button-primary px-4 py-2 text-sm"
              style={{
                background: type === 'error' ? 'var(--accent-danger)' :
                           type === 'warning' || type === 'confirm' ? 'var(--accent-warning)' :
                           type === 'success' ? 'var(--accent-success)' :
                           'var(--accent-primary)',
              }}
              autoFocus
            >
              {type === 'confirm' ? confirmText : 'OK'}
            </button>
          </div>
        </div>
      </div>
    </>
  );
};