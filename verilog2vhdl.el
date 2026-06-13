;; -*- lexical-binding: t -*-
;;; verilog2vhdl.el --- Run verilog2vhdl on a region

;; Copyright (C) 2025  hannes

;; Author: hannes
;; Keywords: verilog vhdl converter
;; Package-Requires: ((emacs "27.1"))

;; Usage:
;;   C-u M-x verilog2vhdl-region          ; prompt for executable path
;;   M-x verilog2vhdl-region              ; uses verilog2vhdl from PATH
;;
;; The converted VHDL is copied to the kill ring via vhdl-port-copy
;; (from vhdl-mode) so it can be pasted directly into a VHDL source
;; buffer.  Input is fed via stdin (no temporary files).

;;; Commentary:
;;
;; Requires:
;;   - `verilog2vhdl' on your $PATH (or use `C-u' to specify a path)
;;   - `vhdl-mode' (optional — when loaded, `vhdl-port-copy' is used
;;     to extract the entity ports into the kill ring)

;;; Code:

(require 'cl-lib)

(defcustom verilog2vhdl-program nil
  "Path to the `verilog2vhdl' executable.
If nil, `verilog2vhdl' is looked up in $PATH via `executable-find'.
Setting this to an absolute path avoids PATH lookup on every call."
  :type '(choice (string :tag "Absolute path to executable")
                 (const :tag "Find in PATH" nil))
  :group 'verilog2vhdl)

(defun verilog2vhdl-region (start end)
  "Convert the Verilog text between START and END to VHDL.

The VHDL output is copied to the kill ring so it can be pasted
into any VHDL source buffer.  Input is fed via stdin (no temp file).

With a universal prefix argument (C-u), you are prompted for the
path to the verilog2vhdl executable.

When `vhdl-mode' is loaded, `vhdl-port-copy' is used to extract
the entity ports into the kill ring."
  (interactive
   (if (called-interactively-p 'interactive)
       (if current-prefix-arg
           (list (region-beginning)
                 (region-end)
                 (read-file-name "verilog2vhdl executable: "
                                 (or verilog2vhdl-program
                                     (executable-find "verilog2vhdl"))))
         (list (region-beginning) (region-end)))
     (list (point-min) (point-max))))
  (let* ((executable (or verilog2vhdl-program
                         (executable-find "verilog2vhdl")))
         (use-entity-only (functionp 'vhdl-port-copy))
         (text (buffer-substring-no-properties start end))
         (result ""))
    (unless executable
      (error "verilog2vhdl not found — use C-u M-x verilog2vhdl-region to specify a path"))
    ;; Feed region text via stdin (no temp file), capture stdout into
    ;; the temp buffer.  When START is a string, call-process-region
    ;; sends that string directly as stdin (END is ignored).
    ;; (fn START END PROGRAM &optional DELETE BUFFER DISPLAY &rest ARGS)
    (with-temp-buffer
      (let ((exit-code
             (apply #'call-process-region
                    text nil executable
                    nil (current-buffer)
                    (if use-entity-only '("--entity-only")))))
        (unless (zerop exit-code)
          (user-error "verilog2vhdl exited with code %d" exit-code))
        (setq result (buffer-string))))
    ;; Copy to kill ring — use vhdl-port-copy if available.
    (if (functionp 'vhdl-port-copy)
        (with-temp-buffer
          (vhdl-mode)
          (insert result)
          (beginning-of-buffer)
	  (search-forward "entity")
          (vhdl-port-copy))
      (kill-new result))
    (message "Verilog converted to VHDL and copied to kill ring")))

(defun verilog2vhdl-buffer ()
  "Convert the entire current buffer from Verilog to VHDL.

This is a convenience wrapper around `verilog2vhdl-region' that
operates on the full buffer contents."
  (interactive)
  (verilog2vhdl-region (point-min) (point-max)))

(provide 'verilog2vhdl)
;;; verilog2vhdl.el ends here
