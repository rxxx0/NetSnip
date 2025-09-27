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
        className="fixed inset-0 z-[9998] bg-black/60 backdrop-blur-md animate-fade-in"
        style={{ animationDuration: '200ms' }}
        onClick={type !== 'confirm' ? onClose : undefined}
      />

      {/* Modal */}
      <div className="fixed inset-0 z-[9999] flex items-center justify-center p-4">
        <div
          className="neu-card max-w-md w-full p-8 animate-scale-in relative overflow-hidden"
          style={{
            animationDuration: '300ms',
            boxShadow: '0 25px 50px rgba(0, 0, 0, 0.4)',
          }}
        >
          {/* Background accent */}
          <div
            className="absolute top-0 left-0 w-full h-1"
            style={{
              background: getAccentColor(),
            }}
          />

          {/* Icon and Title */}
          <div className="flex flex-col items-center text-center mb-6">
            <div
              className="w-14 h-14 rounded-2xl flex items-center justify-center mb-4"
              style={{
                background: `linear-gradient(135deg, ${getAccentColor()}15, ${getAccentColor()}25)`,
                boxShadow: `0 8px 16px ${getAccentColor()}20`,
              }}
            >
              {getIcon()}
            </div>
            <h3 className="text-xl font-semibold text-text-primary">
              {title}
            </h3>
          </div>

          {/* Message */}
          <p className="text-sm text-text-secondary text-center mb-8 leading-relaxed px-4">
            {message}
          </p>

          {/* Actions */}
          <div className="flex gap-3">
            {type === 'confirm' && (
              <button
                onClick={() => {
                  if (onCancel) onCancel();
                  onClose();
                }}
                className="neu-button flex-1 py-2.5 text-sm font-medium hover:transform hover:scale-[1.02] transition-all"
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
              className="flex-1 py-2.5 text-sm font-medium text-white rounded-lg transition-all hover:transform hover:scale-[1.02]"
              style={{
                background: type === 'error' ? 'var(--accent-danger)' :
                           type === 'warning' || type === 'confirm' ? 'var(--accent-warning)' :
                           type === 'success' ? 'var(--accent-success)' :
                           'var(--accent-primary)',
                boxShadow: `0 4px 12px ${type === 'error' ? 'var(--accent-danger)' :
                           type === 'warning' || type === 'confirm' ? 'var(--accent-warning)' :
                           type === 'success' ? 'var(--accent-success)' :
                           'var(--accent-primary)'}40`,
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