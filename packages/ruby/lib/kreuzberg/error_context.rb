# frozen_string_literal: true

require 'json'

module Kreuzberg
  # ErrorContext module provides access to FFI error introspection functions.
  #
  # This module retrieves detailed error and panic context information from the native
  # Rust core. It allows inspection of the last error that occurred during extraction,
  # including panic information with file, line, function, and timestamp details.
  module ErrorContext
    class << self
      # Get the error code of the last operation.
      #
      # Returns the error code from the last FFI call. Returns 0 (SUCCESS) if no error
      # occurred or if introspection fails.
      #
      # @return [Integer] Error code constant (ERROR_CODE_* values), or 0 on success
      #
      # @example Check last error
      #   code = Kreuzberg::ErrorContext.last_error_code
      #   case code
      #   when Kreuzberg::ERROR_CODE_IO
      #     puts "I/O error occurred"
      #   when Kreuzberg::ERROR_CODE_PARSING
      #     puts "Parsing error occurred"
      #   else
      #     puts "Success or unknown error"
      #   end
      def last_error_code
        Kreuzberg._last_error_code_native
      rescue StandardError
        0
      end

      # Get panic context information from the last error.
      #
      # Returns a {Errors::PanicContext} object containing detailed information about
      # the last panic that occurred in the Rust core. Includes file path, line number,
      # function name, error message, and timestamp.
      #
      # @return [Errors::PanicContext, nil] Panic context if a panic occurred, nil otherwise
      #
      # @example Get panic details
      #   panic = Kreuzberg::ErrorContext.last_panic_context
      #   if panic
      #     puts "Panic at #{panic.file}:#{panic.line} in #{panic.function}"
      #     puts "Message: #{panic.message}"
      #     puts "Time: #{panic.timestamp_secs}"
      #   end
      def last_panic_context
        json_str = Kreuzberg._last_panic_context_json_native
        return nil unless json_str

        Errors::PanicContext.from_json(json_str)
      rescue StandardError
        nil
      end

      # Get panic context as raw JSON string.
      #
      # Returns the panic context information as a JSON string for raw access or
      # custom parsing. Returns nil if no panic has occurred.
      #
      # @return [String, nil] JSON-serialized panic context, or nil if no panic
      #
      # @example Get raw JSON panic context
      #   json = Kreuzberg::ErrorContext.last_panic_context_json
      #   if json
      #     panic_data = JSON.parse(json)
      #     puts panic_data
      #   end
      def last_panic_context_json
        Kreuzberg._last_panic_context_json_native
      rescue StandardError
        nil
      end
    end
  end
end
