title: super
date: 2016-01-03
---

# Hello World!
## Heading

[Text](./test2.html)

## Some code

    class FailedJob < Job

      attr_reader :id,
                  :failed_at,
                  :exception,
                  :error,
                  :backtrace,
                  :worker,
                  :retried_at

      def initialize(attributes={})
        super(attributes)
        @id         = attributes[:id]
        @failed_at  = attributes[:failed_at]
        @exception  = attributes[:exception]
        @error      = attributes[:error]
        @backtrace  = attributes[:backtrace]
        @worker     = attributes[:worker]
        @retried_at = attributes[:retried_at]
      end
    end
