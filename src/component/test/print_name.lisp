(defun get-name ()
    "Duckie")

(defun print-name ()
    (format t "hello, ~a", (get-name)))