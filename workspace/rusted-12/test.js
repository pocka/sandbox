const {Some} = require('rusted')

const {expect} = require('chai')

describe('test', function () {
  it('Some(5) is not Some(6)', function () {
    expect(Some(5)).not.to.deep.equal(Some(6))
  })
})
