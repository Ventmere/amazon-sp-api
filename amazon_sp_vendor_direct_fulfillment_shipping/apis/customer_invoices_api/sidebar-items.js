initSidebarItems({"enum":[["GetCustomerInvoiceError","struct for typed errors of method [`get_customer_invoice`]"],["GetCustomerInvoicesError","struct for typed errors of method [`get_customer_invoices`]"]],"fn":[["get_customer_invoice","Returns a customer invoice based on the purchaseOrderNumber that you specify.  Usage Plans:  | Plan type | Rate (requests per second) | Burst | | –– | –– | –– | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation."],["get_customer_invoices","Returns a list of customer invoices created during a time frame that you specify. You define the  time frame using the createdAfter and createdBefore parameters. You must use both of these parameters. The date range to search must be no more than 7 days.  Usage Plans:  | Plan type | Rate (requests per second) | Burst | | –– | –– | –– | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation."]]});